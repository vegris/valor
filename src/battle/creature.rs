use std::collections::VecDeque;
use std::time::Instant;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Point;

use crate::util::AnyError;
use crate::enumerations::Creature;
use crate::resources::{ResourceRegistry, AnimationType};
use crate::time_progress::{Tweening, Animation};

use super::GridPos;


pub struct CreatureStack {
    creature: Creature,
    current_pos: Point,

    current_tweening: Option<Tweening>,
    tweening_queue: VecDeque<Tweening>,

    animation_type: AnimationType,
    animation_progress: f32,

    current_animation: Option<Animation>,
    animation_queue: VecDeque<Animation>
}

impl CreatureStack {
    pub fn new(creature: Creature, grid_pos: GridPos) -> Self {
        Self {
            creature,
            current_pos: grid_pos.draw_pos(),

            current_tweening: None,
            tweening_queue: VecDeque::new(),

            animation_type: AnimationType::Standing,
            animation_progress: 0.,

            current_animation: None,
            animation_queue: VecDeque::new()
        }
    }

    pub fn update(&mut self, now: Instant) {
        if let Some(tweening) = &self.current_tweening {
            tweening.update(now, &mut self.current_pos);
            if tweening.is_finished(now) {
                self.current_tweening = None;
            }
        }
        if self.current_tweening.is_none() {
            self.current_tweening = self.tweening_queue.pop_front();
        }

        if let Some(animation) = &mut self.current_animation {
            animation.update(now, &mut self.animation_type, &mut self.animation_progress);
            if animation.is_finished(now) {
                self.current_animation = None;
            }
        }

        if self.current_animation.is_none() {
            self.current_animation = self.animation_queue.pop_front();
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        let spritesheet = rr.get_creature_container(self.creature);
        let sprite = spritesheet.get_sprite(self.animation_type, self.animation_progress).unwrap();
        
        let draw_rect = sprite.draw_rect(self.current_pos);
        let texture = sprite.surface().as_texture(tc)?;

        canvas.copy(&texture, None, draw_rect)?;

        Ok(())
    }

    pub fn current_pos(&self) -> Point {
        self.current_pos
    }

    pub fn push_tweening(&mut self, tweening: Tweening) {
        self.tweening_queue.push_back(tweening);
    }

    pub fn push_animation(&mut self, animation: Animation) {
        self.animation_queue.push_back(animation);
    }
}
