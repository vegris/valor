use std::error::Error;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use crate::registry::ResourceRegistry;
use crate::gridpos::GridPos;
use crate::graphics::cursors::Cursor;
use crate::command::Command;
use crate::pathfinding;

use super::BattleState;
use super::input::FrameData;

impl<'a> BattleState<'a> {
    pub fn draw(
        &self,
        frame_data: FrameData,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>,
        font: &Font
    ) -> Result<(), Box<dyn Error>> {

        // Рисуем поле боя
        canvas.copy(&self.battlefield, None, Rect::new(0, 0, 800, 556))?;

        // Рисуем клетки на поле
        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let draw_rect = GridPos::new(x, y).bounding_rect();
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }

        let mut highlighted_cells = vec![];

        // Выделяем клетку под курсором
        if let Some(cell) = frame_data.current_hover {
            highlighted_cells.push(cell);
        }

        // Выставляем курсор под ситуацию
        let cursor = choose_cursor(self, &frame_data);
        self.cursors.set(cursor);

        if let Some(command) = frame_data.potential_lmb_command {
            match command {
                // Выделяем потенциальную позицию атакующего стека в случае атаки
                Command::Attack { attack_position, attack_direction } => {
                    let current_side = self.get_current_side();
                    let current_stack = self.get_current_stack();

                    let potential_position = pathfinding::unit_position_for_attack(
                        attack_position, attack_direction, current_side, current_stack.creature.is_wide()
                    );

                    if let Some(pos) = potential_position {
                        for cell in current_stack.get_occupied_cells_for(current_side, pos) {
                            highlighted_cells.push(cell);
                        }
                    }
                },
                // Выделяем потенциальную позицию после перемещения (объединить в функцию с верхней)
                Command::Move { destination } => {
                    let current_side = self.get_current_side();
                    let current_stack = self.get_current_stack();

                    let potential_tail_pos = current_stack.tail_from_head(current_side, destination);
                    for cell in current_stack.get_occupied_cells_for(current_side, potential_tail_pos) {
                        highlighted_cells.push(cell);
                    }
                }
                _ => {}
            }
        }

        for cell in &self.reachable_cells {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        highlighted_cells.sort();
        highlighted_cells.dedup();
        for cell in highlighted_cells {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        // Рисуем существ
        for handle in self.units() {
            let stack = self.get_stack(handle);
            stack.draw(canvas, rr, tc, false, handle.side, &self.stack_count_bg, &font)?;
        }

        let current_stack = self.get_current_stack();
        current_stack.draw(canvas, rr, tc, true, self.current_stack.side, &self.stack_count_bg, &font)?;

        Ok(())
    }
}

fn choose_cursor(state: &BattleState, frame_data: &FrameData) -> Cursor {
    let current_stack = state.get_current_stack();

    if let Some(command) = frame_data.potential_lmb_command {
        match command {
            Command::Move { .. } => {
                if current_stack.creature.is_flying() {
                    Cursor::Fly
                } else {
                    Cursor::Run
                }
            },
            Command::Attack { attack_direction, .. } => {
                Cursor::from_attack_direction(attack_direction)
            },
            Command::Shoot { .. } => Cursor::Arrow,
            _ => unreachable!()
        }
    } else {
        Cursor::Pointer
    }
}