use std::error::Error;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::registry::ResourceRegistry;
use crate::gridpos::GridPos;

use super::BattleState;

impl<'a> BattleState<'a> {
    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>
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

        let pos = GridPos::new(6, 6);
        canvas.set_draw_color(Color::RED);
        // canvas.draw_rect(pos.draw_rect())?;
        canvas.draw_point(pos.center())?;

        let top_right = pos.center().offset((GridPos::CELL_WIDTH / 2) as i32, -((GridPos::CELL_VERTICAL / 2) as i32));
        canvas.draw_point(top_right)?;
        canvas.set_draw_color(Color::BLACK);

        // Выделяем клетку под курсором
        if let Some(pos) = &self.current_hover {
            canvas.copy(&self.grid_cell_shadow, None, pos.bounding_rect())?;
        }

        // Рисуем существ
        for side in &self.sides {
            for unit in side {
                unit.draw(canvas, rr, tc, false)?;
            }
        }

        let current_stack = self.get_current_stack();
        current_stack.draw(canvas, rr, tc, true)?;

        Ok(())
    }
}
