use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use strum::IntoEnumIterator;

use crate::battlestate::{BattleState, Side, StackHandle};
use crate::command::Command;
use crate::error::AnyWay;
use crate::event::Event;
use crate::grid::GridPos;
use crate::input::FrameData;
use crate::map::Map;
use crate::pathfinding;
use crate::registry::ResourceRegistry;

mod animations;
mod cursors;
pub mod spritesheet;
pub mod stack;
pub mod statics;

use cursors::{Cursor, Cursors};
pub use statics::Statics;

use self::statics::{Buttons, StaticTexture};
use spritesheet::hero;

use self::animations::AnimationState;

pub struct Animations(Map<StackHandle, AnimationState>);

impl Animations {
    pub fn create(state: &BattleState, rr: &mut ResourceRegistry) -> Self {
        let animations = state
            .units()
            .into_iter()
            .map(|handle| {
                let stack = state.get_stack(handle);
                let animation = AnimationState::new(stack.creature, stack.head, rr);

                (handle, animation)
            })
            .collect();

        Self(Map(animations))
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        for animation_state in self.0 .0.values_mut() {
            animation_state.update(dt, rr);
        }
    }

    pub fn is_animating(&self) -> bool {
        self.0 .0.values().any(|a| a.is_animating())
    }
}

pub fn process_events(
    state: &BattleState,
    events: Vec<Event>,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    for event in events {
        animations::process_event(state, event, animations, rr);
    }
}

pub fn draw(
    state: &BattleState,
    frame_data: &FrameData,
    animations: &Animations,
    canvas: &mut WindowCanvas,
    rr: &mut ResourceRegistry,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
) -> AnyWay {
    draw_battlefield(canvas, statics)?;

    draw_heroes(canvas, tc, statics)?;

    let is_animating = animations.is_animating();

    set_cursor(&statics.cursors, state, frame_data, is_animating);

    if !is_animating {
        highlight_cells(canvas, statics, state.reachable_cells())?;
        highlight_cells(
            canvas,
            statics,
            &gather_highlighted_cells(state, frame_data),
        )?;
    }

    draw_units(canvas, tc, statics, rr, state, animations)?;

    draw_menu(canvas, tc, statics)?;

    Ok(())
}

fn draw_battlefield(canvas: &mut WindowCanvas, statics: &Statics) -> AnyWay {
    canvas.copy(
        statics.textures.get(StaticTexture::Battlefield),
        None,
        sdl2::rect::Rect::new(0, 0, 800, 556),
    )?;

    for x in GridPos::X_RANGE {
        for y in GridPos::Y_RANGE {
            canvas.copy(
                statics.textures.get(StaticTexture::GridCell),
                None,
                GridPos::new(x, y).bounding_rect(),
            )?;
        }
    }

    Ok(())
}

fn draw_heroes(
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
) -> AnyWay {
    for side in Side::iter() {
        if let Some(hero) = &statics.heroes[side as usize] {
            hero.draw(canvas, tc, side, hero::AnimationType::Idle, 0)?;
        }
    }

    Ok(())
}

fn highlight_cells(canvas: &mut WindowCanvas, statics: &Statics, cells: &[GridPos]) -> AnyWay {
    for cell in cells {
        canvas.copy(
            statics.textures.get(StaticTexture::GridCellShadow),
            None,
            cell.bounding_rect(),
        )?;
    }

    Ok(())
}

fn draw_units(
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
    rr: &mut ResourceRegistry,
    state: &BattleState,
    animations: &Animations,
) -> AnyWay {
    let mut units = state.units();
    units.sort_unstable_by_key(|&handle| {
        let alive = state.get_stack(handle).is_alive();
        let position = animations.0 .0[&handle].position;

        (alive, (position.y, position.x))
    });

    let is_animating = animations.is_animating();

    for handle in units {
        let is_current = state.is_current(handle) && !is_animating;
        let stack = state.get_stack(handle);
        let animation_state = animations.0 .0.get(&handle).unwrap();
        stack::draw(stack, animation_state, canvas, rr, tc, is_current, statics)?;
    }

    Ok(())
}

fn draw_menu(
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
) -> AnyWay {
    canvas.copy(
        statics.textures.get(StaticTexture::MenuBackground),
        None,
        Rect::new(1, 555, 800, 44),
    )?;

    let buttons = [
        (Buttons::Settings, 4),
        (Buttons::Surrender, 55),
        (Buttons::Retreat, 106),
        (Buttons::AutoBattle, 157),
        (Buttons::BookOfMagic, 646),
        (Buttons::Wait, 697),
        (Buttons::Defend, 748),
    ];

    for (button, x) in buttons {
        let sprite = statics.ui[button]
            .get_sprite(spritesheet::button_state::ButtonState::Base, 0)
            .unwrap();
        let texture = sprite.surface.as_texture(tc)?;

        canvas.copy(
            &texture,
            None,
            Rect::new(x, 560, sprite.width, sprite.height),
        )?;
    }

    Ok(())
}

fn set_cursor(cursors: &Cursors, state: &BattleState, frame_data: &FrameData, is_animating: bool) {
    if is_animating {
        cursors.get(Cursor::Pointer).set();
        return;
    }

    let cursor = if let Some(command) = frame_data.potential_lmb_command {
        match command {
            Command::Move { .. } => {
                if state.get_current_stack().creature.is_flying() {
                    Cursor::Fly
                } else {
                    Cursor::Run
                }
            }
            Command::Attack(command) => Cursor::from_attack_direction(command.attack_direction),
            Command::Shoot { .. } => Cursor::Arrow,
            _ => unreachable!(),
        }
    } else {
        Cursor::Pointer
    };

    let sdl_cursor = cursors.get(cursor);
    sdl_cursor.set();
}

fn gather_highlighted_cells(state: &BattleState, frame_data: &FrameData) -> Vec<GridPos> {
    let mut highlighted_cells = vec![];

    if let Some(cell) = frame_data.current_hover {
        highlighted_cells.push(cell);
    }

    if let Some(command) = frame_data.potential_lmb_command {
        match command {
            // Выделяем потенциальную позицию атакующего стека в случае атаки
            Command::Attack(command) => {
                let current_stack = state.get_current_stack();
                let current_side = current_stack.side;

                let potential_position = pathfinding::unit_position_for_attack(
                    command.attack_position,
                    command.attack_direction,
                    current_side,
                    current_stack.creature.is_wide(),
                );

                if let Some(pos) = potential_position {
                    let occupied_cells = pathfinding::get_occupied_cells_for(
                        current_stack.creature,
                        current_side,
                        pos,
                    );

                    if let Some(cells) = occupied_cells {
                        highlighted_cells.extend(cells)
                    }

                    let handle = state.find_unit_for_cell(command.attack_position).unwrap();
                    let target_creature = state.get_stack(handle);

                    for cell in target_creature.get_occupied_cells() {
                        highlighted_cells.push(cell);
                    }
                }
            }
            // Выделяем потенциальную позицию после перемещения (объединить в функцию с верхней)
            Command::Move(command) => {
                let current_stack = state.get_current_stack();

                let occupied_cells = pathfinding::get_occupied_cells_for(
                    current_stack.creature,
                    current_stack.side,
                    command.destination,
                );

                if let Some(cells) = occupied_cells {
                    highlighted_cells.extend(cells);
                }
            }
            _ => {}
        }
    }

    highlighted_cells.sort();
    highlighted_cells.dedup();

    highlighted_cells
}
