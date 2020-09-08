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
    grid_pos: GridPos,

    draw_pos: Point,
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
            grid_pos,

            draw_pos: grid_pos.draw_pos(),
            animation_type: AnimationType::Standing,
            animation_progress: 0.0,

            current_animation: None,
            animation_queue: VecDeque::new(),

            face_left
        }
    }

    pub fn update(&mut self, dt: Duration) {
        if let Some(animation) = &mut self.current_animation {
            if let Some(next_animation) = self.animation_queue.front_mut() {
                // Специальный случай для looping анимаций
                // Если есть чем заменить - меняем
                // Если следующая в очереди анимация ещё на delay,
                // то обновляем delay
                if animation.is_looping() {
                    if next_animation.is_delayed() {
                        next_animation.update(dt);
                    } else {
                        self.take_new_animation();
                    }
                }
            }
        }

        if let Some(animation) = &mut self.current_animation {
            animation.update(dt);
            if let Some(progress) = animation.progress() {
                self.animation_progress = progress;
            }
            if animation.is_finished() {
                animation.at_end().map(|function| function(self));
                self.current_animation = None;
            }
        } else {
            self.take_new_animation();
        }
    }

    fn take_new_animation(&mut self) {
        self.current_animation = self.animation_queue.pop_front();
        if let Some(animation) = &self.current_animation {
           self.animation_type = animation.animation_type();
           self.animation_progress = 0.0;
        }
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

        let draw_rect = sprite.draw_rect(self.draw_pos, self.face_left);
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

    pub fn grid_pos(&self) -> GridPos {
        self.grid_pos
    }

    pub fn set_grid_pos(&mut self, pos: GridPos) {
        self.grid_pos = pos;
    }

    pub fn draw_pos(&self) -> Point {
        self.draw_pos
    }

    pub fn set_draw_pos(&mut self, pos: Point) {
        self.draw_pos = pos
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
