use std::time::Instant;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::enumerations::{Battlefield, Creature, Misc};
use crate::resources::ResourceRegistry;
use crate::util::AnyError;

use super::GridPos;
use super::creature::CreatureStack;
use crate::graphics::choreographer::animate_move_unit;

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
        let mut battlestate = Self {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?,

            creatures: vec![
                CreatureStack::new(Creature::Champion, GridPos::new(5, 9))
            ]

        };

        let champion_path = vec![
            GridPos::new(6, 9),
            GridPos::new(7, 9),
            GridPos::new(8, 9),
            GridPos::new(9, 9),
            GridPos::new(10, 9),
            GridPos::new(11, 9),
            GridPos::new(12, 9),
            GridPos::new(13, 9),
        ];
        animate_move_unit(&mut battlestate, rr, 0, champion_path, Instant::now());

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

    pub fn get_unit_mut(&mut self, unit_index: usize) -> &mut CreatureStack {
        &mut self.creatures[unit_index]
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) -> Result<(), AnyError> {
        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let draw_rect = GridPos::new(x, y).draw_rect();
                canvas.copy(&self.grid_cell_shadow, None, draw_rect)?;
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }
        Ok(())
    }
}
