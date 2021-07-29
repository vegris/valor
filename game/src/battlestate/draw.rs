use std::error::Error;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use crate::registry::ResourceRegistry;
use crate::gridpos::GridPos;
use crate::graphics::cursors::Cursor;
use crate::command::CommandFieldless;

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

        // Выделяем клетку под курсором
        if let Some(cell) = frame_data.current_hover {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        // Выставляем курсор под ситуацию
        let cursor = choose_cursor(self, frame_data);
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

fn choose_cursor(state: &BattleState, frame_data: FrameData) -> Cursor {
    let current_stack = state.get_current_stack();

    if let Some(command) = frame_data.potential_lmb_command {
        match command.fieldless() {
            CommandFieldless::Move => {
                if current_stack.creature.is_flying() {
                    Cursor::Fly
                } else {
                    Cursor::Run
                }
            },
            CommandFieldless::Attack => {
                let attack_direction = frame_data.attack_direction.unwrap();
                Cursor::from_attack_direction(attack_direction)
            },
            CommandFieldless::Shoot => Cursor::Arrow,
            _ => unreachable!()
        }
    } else {
        Cursor::Pointer
    }
}