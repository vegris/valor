use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::battlestate::BattleState;
use crate::command::Command;
use crate::grid::GridPos;
use crate::input::FrameData;
use crate::pathfinding;
use crate::registry::ResourceRegistry;

pub mod creature;
mod cursors;
pub mod hero;
pub mod stack;
pub mod statics;

use cursors::{Cursor, Cursors};
pub use statics::Statics;

use self::statics::StaticTexture;

pub fn draw(
    state: &BattleState,
    frame_data: &FrameData,
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

    let sprite = statics
        .hero
        .get_sprite(hero::AnimationType::Casting, 0.7)
        .unwrap();
    let texture = sprite.surface().as_texture(tc)?;

    canvas.copy(&texture, None, sprite.draw_rect(Point::new(50, 75)))?;
    canvas.copy_ex(
        &texture,
        None,
        sprite.draw_rect(Point::new(785, 75)),
        0.0,
        None,
        true,
        false,
    )?;

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
        stack::draw(
            stack,
            canvas,
            rr,
            tc,
            is_current,
            statics.textures.get(StaticTexture::StackCountBackground),
            &statics.font,
        )?;
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(stack.head.bounding_rect())?;
    }

    canvas.set_draw_color(Color::BLACK);

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
