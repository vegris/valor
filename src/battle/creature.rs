use std::time::Instant;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};

use crate::util::AnyError;
use crate::enumerations::Creature;
use crate::resources::ResourceRegistry;
use crate::tweening::Tweening;
use crate::animation::Animation;

use super::GridPos;

#[derive(Clone, Copy)]
pub enum FacingDirection {
    Left,
    Right
}

pub struct CreatureStack {
    creature: Creature,
    grid_pos: GridPos,

    animation: Animation,
    tweening: Option<Tweening>,

    facing_direction: FacingDirection,
}

impl CreatureStack {
    pub fn new(creature: Creature, grid_pos: GridPos, facing_direction: FacingDirection, rr: &mut ResourceRegistry) -> Self {
        let tweening = Tweening::new(grid_pos.draw_point(), GridPos::new(8, 8).draw_point());
        Self {
            creature,
            grid_pos,
            animation: Animation::default(creature, rr),
            tweening: Some(tweening),
            facing_direction
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.animation.update(now);

        if let Some(tweening) = &mut self.tweening {
            if tweening.is_finished() {
                self.tweening = None
            } else {
                tweening.update(now);
            }
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        let sprite = self.animation.get_sprite(rr);
        
        let draw_point =
            if let Some(tweening) = &self.tweening {
                tweening.draw_point() 
            } else {
                self.grid_pos.draw_point()
            };
        
        let draw_rect = sprite.draw_rect(draw_point);
        let texture = sprite.surface().as_texture(tc)?;

        match self.facing_direction {
            FacingDirection::Left => canvas.copy(&texture, None, draw_rect)?,
            FacingDirection::Right => canvas.copy_ex(&texture, None, draw_rect, 0., None, true, false)?
        };

        Ok(())
    }
}
