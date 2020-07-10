use std::time::Duration;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::pixels::Palette;

use crate::enumerations::{Battlefield, Creature, Misc};
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
    grid_cell_shadow: Texture<'a>
}

impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let graphics = Graphics {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?
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
        self.draw_grid(canvas)?;

        // Рисуем существо
        let creature_def = rr.get_creature_container(Creature::Champion);
        let def_sprite = creature_def.names2sprites.values_mut().next().unwrap();
        let palette = Palette::with_colors(&creature_def.colors)?;
        def_sprite.surface.set_palette(&palette)?;
        let texture = def_sprite.surface.as_texture(tc)?;
        canvas.copy(&texture, None, Rect::new(400, 400, def_sprite.width, def_sprite.height))?;

        Ok(())
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) -> Result<(), AnyError> {
        let grid_texture = &self.graphics.grid_cell;
        let (grid_width, grid_height) = (45, 52);
        let grid_vertical_side = 32;

        // Рисует нечётные ряды
        let (odd_start_x, odd_start_y) = (81, 86);
        for x in 0..15 {
            for y in 0..6 {
                let x_pos = odd_start_x + x * grid_width;
                let y_pos = odd_start_y + y * (grid_height + grid_vertical_side);
                let draw_rect = Rect::new(x_pos as i32, y_pos as i32, grid_width, grid_height);
                canvas.copy(&grid_texture, None, draw_rect)?
            }
        }
        // Рисуем нечётные ряды
        let (even_start_x, even_start_y) = (59, 129);
        for x in 0..15 {
            for y in 0..5 {
                let x_pos = even_start_x + x * grid_width;
                let y_pos = even_start_y + y * (grid_height + grid_vertical_side);
                let draw_rect = Rect::new(x_pos as i32, y_pos as i32, grid_width, grid_height);
                canvas.copy(&grid_texture, None, draw_rect)?;
            }
        }
        Ok(())
    }
}
