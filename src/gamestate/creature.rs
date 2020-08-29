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
use crate::graphics::animations::{Tweening, Animation};

use super::GridPos;


pub struct CreatureStack {
    creature: Creature,
    current_pos: Point,

    current_tweening: Option<Tweening>,
    tweening_queue: VecDeque<Tweening>,

    animation_progress: f32,

    current_animation: Option<Animation>,
    animation_queue: VecDeque<Animation>,

    face_left: bool
}

impl CreatureStack {
    pub fn new(creature: Creature, grid_pos: GridPos, face_left: bool) -> Self {
        Self {
            creature,
            current_pos: grid_pos.draw_pos(),

            current_tweening: None,
            tweening_queue: VecDeque::new(),

            animation_progress: 0.,

            current_animation: None,
            animation_queue: VecDeque::new(),

            face_left
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
            animation.update(now, &mut self.animation_progress);
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
        let animation_type = self.current_animation.as_ref().unwrap().type_();
        let sprite = spritesheet.get_sprite(animation_type, self.animation_progress).unwrap();
        
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

    pub fn push_tweening(&mut self, tweening: Tweening) {
        self.tweening_queue.push_back(tweening);
    }

    pub fn push_animation(&mut self, animation: Animation) {
        self.animation_queue.push_back(animation);
    }
}
