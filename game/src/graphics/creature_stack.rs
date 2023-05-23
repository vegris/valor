use std::error::Error;
use std::time::Duration;

use gamedata::Creature;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::animations::{AnimationQueue, AnimationState, Tweening};
use crate::battlestate::Side;
use crate::graphics::creature::AnimationType;
use crate::ResourceRegistry;

#[derive(Clone, Debug)]
pub struct CreatureStack {
    pub draw_pos: Point,
    pub animation_queue: AnimationQueue,
}

impl CreatureStack {
    pub fn new(draw_pos: Point) -> Self {
        Self {
            draw_pos,
            animation_queue: AnimationQueue::new(),
        }
    }

    pub fn update(&mut self, creature: Creature, dt: Duration, rr: &mut ResourceRegistry) {
        self.animation_queue.update(dt);
        self.animation_queue.remove_finished();

        self.animation_queue.remove_non_existent(creature, rr);

        if let Some(animation) = self.animation_queue.current() {
            if let Some(Tweening { from, to }) = animation.tween {
                if let AnimationState::Running(progress) = animation.state() {
                    let from_c = from.center();
                    let to_c = to.center();
                    let x = from_c.x + ((to_c.x - from_c.x) as f32 * progress).round() as i32;
                    let y = from_c.y + ((to_c.y - from_c.y) as f32 * progress).round() as i32;
                    self.draw_pos = Point::new(x, y);
                }
            }
        }

        self.animation_queue.add_standing();
    }

    pub fn draw(
        &self,
        logic: &crate::creature_stack::CreatureStack,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>,
        is_selected: bool,
        stack_count_bg: &Texture,
        font: &Font,
    ) -> Result<(), Box<dyn Error>> {
        let spritesheet = rr.get_creature_container(logic.creature);

        let animation_type = if let Some(animation) = self.animation_queue.current() {
            animation.type_
        } else if logic.is_alive() {
            AnimationType::Standing
        } else {
            AnimationType::Death
        };

        let animation_block = spritesheet.animation_block(animation_type).unwrap();

        let animation_index = self.animation_index(animation_block.len());
        let sprite_index = animation_block[animation_index];
        let sprite = &mut spritesheet.sprites[sprite_index];
        if is_selected {
            sprite.turn_selection(&mut spritesheet.colors, true)
        };

        let draw_rect = sprite.draw_rect(self.draw_pos, logic.side);
        let texture = sprite.surface().as_texture(tc)?;

        match logic.side {
            Side::Attacker => canvas.copy(&texture, None, draw_rect)?,
            Side::Defender => canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?,
        };

        if is_selected {
            sprite.turn_selection(&mut spritesheet.colors, false)
        };

        if logic.is_alive() {
            let cell_center = logic.head.bounding_rect().center();
            let draw_center = cell_center.offset(0, 10);
            canvas.copy(stack_count_bg, None, Rect::from_center(draw_center, 30, 11))?;

            let font_surface = font.render(&logic.count.to_string()).solid(Color::BLUE)?;
            let font_texture = font_surface.as_texture(tc)?;

            let mut font_rect = font_surface.rect();
            font_rect.center_on(draw_center);

            canvas.copy(&font_texture, None, font_rect)?;
        }

        Ok(())
    }

    fn animation_index(&self, animation_len: usize) -> usize {
        self.animation_queue
            .current()
            .map(|animation| animation.state())
            .and_then(|state| {
                if let AnimationState::Running(progress) = state {
                    Some(progress)
                } else {
                    None
                }
            })
            .map(|progress| (animation_len as f32 * progress).round() as usize)
            .map(|animation_index| std::cmp::min(animation_index, animation_len - 1))
            .unwrap_or(animation_len - 1)
    }
}
