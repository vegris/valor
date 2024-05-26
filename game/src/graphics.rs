use common::error::AnyWay;
use gamedata::cursors::Cursor;
use gamedata::gui::{ButtonState, Texture};
use gamedata::heroes;
use logic::command::Command;
use logic::gamestate::{GameState, Side};
use logic::grid::GridPos;
use logic::pathfinding;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use strum::IntoEnumIterator;

use crate::animations::Animations;
use crate::input::FrameData;
use crate::resources::ResourceRegistry;
use crate::{animations, gridpos, Graphics, Stage};

pub mod creature;
mod cursors;
mod hero;
pub mod stack;
pub mod statics;

use cursors::Cursors;
pub use statics::Statics;

use self::statics::StaticTexture;

pub fn draw(
    state: &GameState,
    frame_data: &FrameData,
    graphics: &mut Graphics,
    animations: &Animations,
    rr: &mut ResourceRegistry,
    shapes: Vec<(Rect, Texture)>,
    stage: &Stage,
) -> AnyWay {
    let canvas = &mut graphics.canvas;
    let tc = &graphics.texture_creator;
    let statics = &graphics.statics;

    draw_battlefield(canvas, statics)?;

    draw_heroes(canvas, tc, statics)?;

    let is_animating = animations.is_animating();

    set_cursor(&statics.cursors, state, frame_data, is_animating);

    if !is_animating {
        highlight_cells(canvas, statics, state, frame_data)?;
    }

    draw_units(canvas, tc, statics, rr, state, animations)?;

    draw_entities(canvas, tc, rr, animations)?;

    draw_gui(canvas, tc, statics, stage, shapes)?;

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
                gridpos::bounding_rect(GridPos::new(x, y)),
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
            hero.draw(canvas, tc, side, heroes::Animation::Idle, 0)?;
        }
    }

    Ok(())
}

fn highlight_cells(
    canvas: &mut WindowCanvas,
    statics: &Statics,
    state: &GameState,
    frame_data: &FrameData,
) -> AnyWay {
    for cell in state.reachable_cells() {
        highlight_cell(canvas, statics, *cell)?;
    }

    for cell in gather_highlighted_cells(state, frame_data) {
        highlight_cell(canvas, statics, cell)?;
    }

    Ok(())
}

fn highlight_cell(
    canvas: &mut WindowCanvas,
    statics: &Statics,
    cell: GridPos,
) -> Result<(), String> {
    canvas.copy(
        statics.textures.get(StaticTexture::GridCellShadow),
        None,
        gridpos::bounding_rect(cell),
    )
}

fn draw_units(
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
    rr: &mut ResourceRegistry,
    state: &GameState,
    animations: &Animations,
) -> AnyWay {
    let mut units = state.units();
    units.sort_unstable_by_key(|&handle| {
        let alive = state.get_stack(handle).is_alive();
        let position = animations.creature[&handle].position;

        (alive, (position.y, position.x))
    });

    let is_animating = animations.is_animating();

    for handle in units {
        let is_current = state.is_current(handle) && !is_animating;
        let stack = state.get_stack(handle);
        let animation_state = animations.creature.get(&handle).unwrap();
        stack::draw(stack, animation_state, canvas, rr, tc, is_current, statics)?;
    }

    Ok(())
}

fn draw_gui(
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
    stage: &Stage,
    shapes: Vec<(Rect, Texture)>,
) -> AnyWay {
    canvas.copy(
        statics.textures.get(StaticTexture::MenuBackground),
        None,
        Rect::new(0, 556, 800, 44),
    )?;

    for (rect, texture) in shapes.iter() {
        match *texture {
            Texture::Button(button, _state) => {
                let sprite = statics.ui.get(button).get(ButtonState::Base);
                let texture = sprite.surface.as_texture(tc)?;

                canvas.copy(&texture, None, *rect)?;
            }
            Texture::Spell(_) => {}
        }
    }

    if matches!(stage, Stage::SpellBook) {
        canvas.copy(
            statics.textures.get(StaticTexture::SpellBook),
            None,
            sdl2::rect::Rect::new(400 - 310, 2, 620, 595),
        )?;
    }

    for (rect, texture) in shapes.iter() {
        match *texture {
            Texture::Button(..) => {}
            Texture::Spell(spell) => {
                let sprite = statics.spells.get(spell);
                let texture = sprite.surface.as_texture(tc)?;

                canvas.copy(&texture, None, *rect)?;
            }
        }
    }

    Ok(())
}

fn draw_entities(
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    rr: &mut ResourceRegistry,
    animations: &Animations,
) -> AnyWay {
    for animation in animations.entity.iter() {
        let spell_animation = rr.get_spell_animation(animation.spell_animation);
        let frame = spell_animation.frames_count() as f32 * animation.progress.progress();
        let sprite = spell_animation.get_frame(frame as usize).unwrap();
        let texture = sprite.surface.as_texture(tc)?;

        canvas.copy(
            &texture,
            None,
            Rect::new(
                animation.position.0,
                animation.position.1,
                sprite.width,
                sprite.height,
            ),
        )?;
    }
    Ok(())
}

fn set_cursor(cursors: &Cursors, state: &GameState, frame_data: &FrameData, is_animating: bool) {
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
            Command::Attack(command) => cursors::from_attack_direction(command.attack_direction),
            Command::Shoot { .. } => Cursor::Arrow,
            _ => unreachable!(),
        }
    } else {
        Cursor::Pointer
    };

    let sdl_cursor = cursors.get(cursor);
    sdl_cursor.set();
}

fn gather_highlighted_cells(state: &GameState, frame_data: &FrameData) -> Vec<GridPos> {
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
