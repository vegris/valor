use std::error::Error;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use crate::registry::ResourceRegistry;
use crate::gridpos::GridPos;
use crate::graphics::cursors::Cursor;

use super::BattleState;

impl<'a> BattleState<'a> {
    pub fn draw(
        &self,
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

        // Выделяем клетку под курсором
        if let Some(cell) = self.current_hover {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        // Выставляем курсор под ситуацию
        let cursor = choose_cursor(self);
        self.cursors.set(cursor);


        for cell in &self.reachable_cells {
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

fn choose_cursor(state: &BattleState) -> Cursor {
    let current_stack = state.get_current_stack();

    if let Some(cell) = state.current_hover {
        let is_reachable = state.reachable_cells.contains(&cell);

        if let Some(handle) = state.find_unit_for_cell(cell) {
            let is_enemy = state.get_current_side() != handle.side;
            let unit = state.get_stack(handle);
            if is_enemy && unit.is_alive() {
                if current_stack.can_shoot() {
                    return Cursor::Arrow;
                }
                if is_reachable {
                    return Cursor::from_hexagon_part(state.hexagon_part.unwrap());
                }
            }
        } else {
            if is_reachable {
                if current_stack.creature.is_flying() {
                    return Cursor::Fly;
                } else {
                    return Cursor::Run;
                }
            }
        }
    }

    return Cursor::Pointer
}