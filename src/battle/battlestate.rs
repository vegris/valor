use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::enumerations::{Battlefield, Creature, Misc};
use crate::resources::{ResourceRegistry, Animation};
use crate::util::AnyError;

use super::GridPos;

pub struct BattleState<'a> {
    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

    creature_animation: CreatureAnimation
}

struct CreatureAnimation {
    creature: Creature,
    type_: Animation,
    current_sprite_index: usize,
    change_at: Instant
}

const ANIMATION_PERIOD: Duration = Duration::from_millis(100);

impl CreatureAnimation {
    pub fn new(creature: Creature, type_: Animation) -> Self {
        CreatureAnimation{
            creature,
            type_,
            current_sprite_index: 0,
            change_at: Instant::now() + ANIMATION_PERIOD
        }
    }

    pub fn update(&mut self, now: Instant) {
        if now >= self.change_at {
            if self.current_sprite_index == 3 {
                self.current_sprite_index = 0;
            } else {
                self.current_sprite_index = self.current_sprite_index + 1;
            }
            let excess_time = now - self.change_at;
            self.change_at = now - excess_time + ANIMATION_PERIOD;
        }
    }

    pub fn get_texture<'a>(&self, rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>, AnyError> {
        let creature_spritesheet = rr.get_creature_container(self.creature);
        let sprite = creature_spritesheet.get_sprite(self.type_, self.current_sprite_index).unwrap();
        let texture = sprite.surface().as_texture(tc)?;
        Ok(texture)
    }

    pub fn get_draw_rect(&self, rr: &mut ResourceRegistry) -> Rect {
        let creature_spritesheet = rr.get_creature_container(self.creature);
        let sprite = creature_spritesheet.get_sprite(self.type_, self.current_sprite_index).unwrap();
        sprite.get_draw_rect_for_grid(GridPos::new(1, 1))
    }
}

impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let battlestate = Self {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?,

            creature_animation: CreatureAnimation::new(Creature::Champion, Animation::Standing)
        };

        Ok(battlestate)
    }

    pub fn update(&mut self, now: Instant) {
        self.creature_animation.update(now);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        // Рисуем поле боя
        canvas.copy(&self.battlefield, None, Rect::new(0, 0, 800, 556))?;
        // Рисуем сетку
        self.draw_grid(canvas)?;

        // Рисуем существо
        let creature_texture = self.creature_animation.get_texture(rr, tc)?;
        let creature_draw_rect = self.creature_animation.get_draw_rect(rr);
        canvas.copy(&creature_texture, None, creature_draw_rect)?;

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
