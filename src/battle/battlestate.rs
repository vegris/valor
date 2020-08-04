use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::enumerations::{Battlefield, Creature, Misc};
use crate::resources::{ResourceRegistry, Animation};
use crate::util::AnyError;

use super::GridPos;
use super::creature::CreatureStack;

pub struct BattleState<'a> {
    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

    creatures: Vec<CreatureStack>
}


impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let battlestate = Self {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?,

            creatures: vec![
                CreatureStack::new(Creature::Peasant, Animation::Standing, GridPos::new(1, 1), rr),
                CreatureStack::new(Creature::Champion, Animation::Moving, GridPos::new(5, 9), rr),
                CreatureStack::new(Creature::Beholder, Animation::Standing, GridPos::new(10, 2), rr)
            ]

        };

        Ok(battlestate)
    }

    pub fn update(&mut self, now: Instant) {
        for creature in &mut self.creatures {
            creature.update(now);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        // Рисуем поле боя
        canvas.copy(&self.battlefield, None, Rect::new(0, 0, 800, 556))?;
        // Рисуем сетку
        self.draw_grid(canvas)?;

        // Рисуем существ
        for creature in &self.creatures {
            creature.draw(canvas, rr, tc)?;
        }

        Ok(())
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) -> Result<(), AnyError> {
        for x in GridPos::X_MIN ..= GridPos::X_MAX {
            for y in GridPos::Y_MIN ..= GridPos::Y_MAX {
                let draw_rect = GridPos::new(x, y).get_draw_rect();
                canvas.copy(&self.grid_cell_shadow, None, draw_rect)?;
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }
        Ok(())
    }
}
