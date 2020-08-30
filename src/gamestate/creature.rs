use std::collections::VecDeque;
use std::time::Instant;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;

use crate::util::AnyError;
use crate::enumerations::Creature;
use crate::resources::ResourceRegistry;
use crate::graphics::creature::AnimationType;
use crate::graphics::animations::CreatureAnimation;

use super::GridPos;


pub struct CreatureStack {
    creature: Creature,
    current_pos: Point,

    animation_type: AnimationType,
    animation_progress: f32,

    current_animation: Option<CreatureAnimation>,
    animation_queue: VecDeque<CreatureAnimation>,

    pub face_left: bool
}

impl CreatureStack {
    pub fn new(creature: Creature, grid_pos: GridPos, face_left: bool) -> Self {
        Self {
            creature,
            current_pos: grid_pos.draw_pos(),

            animation_type: AnimationType::Standing,
            animation_progress: 0.0,

            current_animation: None,
            animation_queue: VecDeque::new(),

            face_left
        }
    }

    pub fn update(&mut self, now: Instant) {
        let maybe_animation = self.current_animation.take();
        if let Some(animation) = maybe_animation {
            animation.update(self, now);
            if animation.is_finished(now) {
                animation.at_end(self);
            } else {
                self.current_animation = Some(animation);
            }
        }

        if self.current_animation.is_none() {
            if let Some(animation) = self.animation_queue.pop_front() {
                animation.at_start(self);
                self.current_animation = Some(animation);
            }
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        let spritesheet = rr.get_creature_container(self.creature);
        let sprite = spritesheet.get_sprite(self.animation_type, self.animation_progress).unwrap();
        
        canvas.set_draw_color(Color::BLUE);
        canvas.fill_rect(Rect::from_center(self.current_pos, 10, 10))?;

        let draw_rect = sprite.draw_rect(self.current_pos, self.face_left);
        let texture = sprite.surface().as_texture(tc)?;

        if self.face_left {
            canvas.copy(&texture, None, draw_rect)?;
        } else {
            canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?;

        }
        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(draw_rect)?;
        canvas.set_draw_color(Color::BLACK);

        Ok(())
    }

    pub fn creature(&self) -> Creature {
        self.creature
    }

    pub fn current_pos(&self) -> Point {
        self.current_pos
    }

    pub fn set_current_pos(&mut self, pos: Point) {
        self.current_pos = pos
    }
    
    pub fn set_animation_type(&mut self, animation_type: AnimationType) {
        self.animation_type = animation_type;
    }

    pub fn set_animation_progress(&mut self, progress: f32) {
        self.animation_progress = progress;
    }

    pub fn push_animation(&mut self, animation: CreatureAnimation) {
        self.animation_queue.push_back(animation);
    }
}
