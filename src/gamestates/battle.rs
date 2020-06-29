use std::time::Duration;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::enumerations::Battlefield;
use crate::resources::ResourceRegistry;
use crate::util::AnyError;

pub struct BattleState {
    battlefield: Battlefield,
}

impl BattleState {
    pub fn new(battlefield: Battlefield) -> Self {
        BattleState { battlefield }
    }

    pub fn update(&mut self, _dt: Duration) {
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        // Рисуем поле боя
        let surface = rr.get_battlefield_surface(self.battlefield);
        let texture = surface.as_texture(&tc)?;
        canvas.copy(&texture, None, Rect::new(0, 0, 800, 556))?;
        // Рисуем сетку
        Ok(())
    }
}
