use std::error::Error;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use crate::registry::ResourceRegistry;
use crate::gridpos::GridPos;
use crate::graphics::cursors::Cursor;

use super::BattleState;

impl<'a> BattleState<'a> {
    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>
    ) -> Result<(), Box<dyn Error>> {

        self.cursors.set(Cursor::Pointer);

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

            if let Some((side, _)) = self.find_unit_for_cell(cell) {
                if self.current_side != side {
                    if self.get_current_stack().can_shoot() {
                        self.cursors.set(Cursor::Arrow);
                    }
                }
            }
        }

        let current_stack = self.get_current_stack();

        let accessible_cells = self.navigation_array.get_reachable_cells(current_stack.speed().into());
        for cell in accessible_cells {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        // Рисуем существ
        for side in &self.sides {
            for unit in side {
                unit.draw(canvas, rr, tc, false)?;
            }
        }
        current_stack.draw(canvas, rr, tc, true)?;

        Ok(())
    }
}
