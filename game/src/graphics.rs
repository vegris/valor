use std::collections::{HashMap, hash_map};
use std::error::Error;

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use strum::IntoEnumIterator;

use crate::battlestate::{BattleState, Side, StackHandle};
use crate::command::Command;
use crate::event::Event;
use crate::grid::GridPos;
use crate::input::FrameData;
use crate::pathfinding;
use crate::registry::ResourceRegistry;

mod animations;
mod cursors;
pub mod spritesheet;
pub mod stack;
pub mod statics;

use cursors::{Cursor, Cursors};
pub use statics::Statics;

use self::statics::StaticTexture;
use spritesheet::hero;

use self::animations::AnimationState;

pub struct Animations(HashMap<StackHandle, AnimationState>);

pub fn create_animations(state: &BattleState, rr: &mut ResourceRegistry) -> Animations {
    let animations = state
        .units()
        .into_iter()
        .map(|handle| {
            let creature = state.get_stack(handle).creature;
            let animation = AnimationState::new(creature, rr);

            (handle, animation)
        })
        .collect();

    Animations(animations)
}

impl Animations {
    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, StackHandle, AnimationState> {
        self.0.values_mut()
    }

    fn get_many_mut<const N: usize>(
        &mut self,
        handles: [StackHandle; N],
    ) -> Option<[&mut AnimationState; N]> {
        use std::mem::MaybeUninit;

        for index in 1..N {
            if handles[index] == handles[index - 1] {
                return None;
            }
        }

        let mut arr: MaybeUninit<[&mut AnimationState; N]> = MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        // SAFETY: We expect `handles` to contain disjunct values that are in bounds of `self`.
        unsafe {
            for (i, handle) in handles.iter().enumerate() {
                if let Some(stack) = self.0.get_mut(handle) {
                    *(*arr_ptr).get_unchecked_mut(i) = &mut *(stack as *mut _);
                } else {
                    return None;
                }
            }

            Some(arr.assume_init())
        }
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
) -> Result<(), Box<dyn Error>> {
    // Рисуем поле боя
    canvas.copy(
        statics.textures.get(StaticTexture::Battlefield),
        None,
        sdl2::rect::Rect::new(0, 0, 800, 556),
    )?;

    for side in Side::iter() {
        if let Some(hero) = &statics.heroes[side as usize] {
            hero.draw(canvas, tc, side, hero::AnimationType::Idle, 0)?;
        }
    }

    // Рисуем клетки на поле
    for x in GridPos::X_RANGE {
        for y in GridPos::Y_RANGE {
            let draw_rect = GridPos::new(x, y).bounding_rect();
            canvas.copy(
                statics.textures.get(StaticTexture::GridCell),
                None,
                draw_rect,
            )?;
        }
    }

    let mut highlighted_cells = vec![];

    // Выделяем клетку под курсором
    if let Some(cell) = frame_data.current_hover {
        highlighted_cells.push(cell);
    }

    set_cursor(&statics.cursors, state, frame_data);

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

    for cell in state.reachable_cells() {
        canvas.copy(
            statics.textures.get(StaticTexture::GridCellShadow),
            None,
            cell.bounding_rect(),
        )?;
    }

    highlighted_cells.sort();
    highlighted_cells.dedup();
    for cell in highlighted_cells {
        canvas.copy(
            statics.textures.get(StaticTexture::GridCellShadow),
            None,
            cell.bounding_rect(),
        )?;
    }

    // Рисуем существ
    let mut units = state.units();
    units.sort_unstable_by_key(|&handle| state.get_stack(handle).head.y);

    for handle in units {
        let is_current = state.is_current(handle);
        let stack = state.get_stack(handle);
        let animation_state = animations.0.get(&handle).unwrap();
        stack::draw(stack, animation_state, canvas, rr, tc, is_current, statics)?;
    }

    Ok(())
}

fn set_cursor(cursors: &Cursors, state: &BattleState, frame_data: &FrameData) {
    let current_stack = state.get_current_stack();

    let cursor = if let Some(command) = frame_data.potential_lmb_command {
        match command {
            Command::Move { .. } => {
                if current_stack.creature.is_flying() {
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
