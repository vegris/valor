use std::time::Duration;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::enumerations::{Battlefield, Misc};
use crate::resources::ResourceRegistry;
use crate::util::AnyError;

pub struct BattleState<'a> {
    logic: Logic,
    graphics: Graphics<'a>
}

struct Logic {
    battlefield: Battlefield
}

// Постоянно используемые текстуры,
// которые нет смысла прокачивать сквозь кэш
struct Graphics<'a> {
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadowed: Texture<'a>
}

impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let graphics = Graphics {
            battlefield: rr.load_pcx(battlefield.filename()).as_texture(&tc)?,
            grid_cell: rr.load_pcx(Misc::CellGrid.filename()).as_texture(&tc)?,
            grid_cell_shadowed: rr.load_pcx(Misc::CellGridShadowed.filename()).as_texture(&tc)?
        };
        let logic = Logic {
            battlefield
        };
        let battlestate = BattleState { 
            logic,
            graphics
        };

        Ok(battlestate)
    }

    pub fn update(&mut self, _dt: Duration) {
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        let Self { logic, graphics } = self;
        // Рисуем поле боя
        canvas.copy(&graphics.battlefield, None, Rect::new(0, 0, 800, 556))?;
        // Рисуем сетку
        canvas.copy(&graphics.grid_cell_shadowed, None, Rect::new(200, 200, 50, 50))?;

        Ok(())
    }
}
