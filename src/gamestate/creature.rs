use std::collections::VecDeque;
use std::time::Duration;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Point;

use crate::util::AnyError;
use crate::enumerations::Creature;
use crate::resources::ResourceRegistry;
use crate::graphics::creature::{AnimationType, CreatureSprite};
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

    pub fn update(&mut self, dt: Duration) {
        let mut current_animation = self.current_animation.take();
        if current_animation.is_none() {
            if let Some(animation) = self.animation_queue.pop_front() {
               animation.at_start(self); 
               current_animation = Some(animation);
            }
        }

        if let Some(ref mut animation) = current_animation {
            animation.update(self, dt);
            if animation.is_finished() {
                animation.at_end(self);
                current_animation = None;
            }
        }

        self.current_animation = current_animation;
    }

    fn get_sprite<'a>(&self, rr: &'a mut ResourceRegistry) -> &'a CreatureSprite {
        let spritesheet = rr.get_creature_container(self.creature);
        let animation_block = spritesheet.get_animation_block(self.animation_type);
        // Номер спрайта в анимации
        let sprite_num = (animation_block.len() as f32 * self.animation_progress).floor() as usize;
        // Индекс спрайта в массиве всех спрайтов
        let sprite_index = animation_block[sprite_num];
        spritesheet.get_sprite(sprite_index)
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        let sprite = self.get_sprite(rr);

        // canvas.set_draw_color(Color::BLUE);
        // canvas.fill_rect(Rect::from_center(self.current_pos, 10, 10))?;

        let draw_rect = sprite.draw_rect(self.current_pos, self.face_left);
        let texture = sprite.surface().as_texture(tc)?;

        if self.face_left {
            canvas.copy(&texture, None, draw_rect)?;
        } else {
            canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?;

        }
        // canvas.set_draw_color(Color::RED);
        // canvas.draw_rect(draw_rect)?;
        // canvas.set_draw_color(Color::BLACK);

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
