use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};

use crate::util::AnyError;
use crate::enumerations::Creature;
use crate::resources::{ResourceRegistry, Animation};

use super::GridPos;

const ADVANCE_PERIODITY: Duration = Duration::from_millis(128);

pub struct CreatureStack {
    creature: Creature,
    animation: Animation,
    grid_pos: GridPos,

    cur_sprite_index: usize,
    sprites_total: usize,
    advance_at: Instant
}

impl CreatureStack {
    pub fn new(creature: Creature, animation: Animation, grid_pos: GridPos, rr: &mut ResourceRegistry) -> Self {
        let creature_spritesheet = rr.get_creature_container(creature);
        let anim_block = creature_spritesheet.get_animation_block(animation).unwrap();
        let sprites_total = anim_block.len();
        dbg!(sprites_total);

        Self {
            creature,
            animation,
            grid_pos,
            cur_sprite_index: 0,
            sprites_total,
            advance_at: Instant::now()
        }
    }

    pub fn update(&mut self, now: Instant) {
        if now >= self.advance_at {
            // Обновляем индекс спрайта
            self.cur_sprite_index = self.cur_sprite_index + 1;
            if self.cur_sprite_index == self.sprites_total {
                self.cur_sprite_index = 0;
            }

            // Обновляем таймер
            self.advance_at = self.advance_at + ADVANCE_PERIODITY;
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError>{
        let sprite = rr.get_creature_container(self.creature).get_sprite(self.animation, self.cur_sprite_index).unwrap();
        let draw_rect = sprite.get_draw_rect_for_grid(&self.grid_pos);
        let texture = sprite.surface().as_texture(tc)?;

        canvas.copy(&texture, None, draw_rect)?;

        Ok(())
    }
}