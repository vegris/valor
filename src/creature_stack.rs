use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Duration;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Point;

use crate::util::AnyError;
use crate::resources::ResourceRegistry;
use crate::graphics::creature::{AnimationType, CreatureSprite};
use crate::graphics::animations::CreatureAnimation;

use super::creature::{Creature, CreatureStats};
use super::gridpos::GridPos;
use super::battlestate::Side;

/// Существо в течение раунда может принимать одно из этих состояний
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreatureTurnState {
    HasTurn,
    Waited,
    NoTurn
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right
}

impl Direction {
    pub fn inversion(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
}

pub struct CreatureStack {
    // Логика
    pub creature: Creature,
    pub count: u32,

    pub current_health: u16,
    pub current_ammo: u8,

    pub position: GridPos,

    pub turn_state: CreatureTurnState,
    pub defending: bool,

    // Графика
    pub draw_pos: Point,
    pub animation_type: AnimationType,
    pub animation_progress: f32,

    pub current_animation: Option<CreatureAnimation>,
    pub animation_queue: VecDeque<CreatureAnimation>,

    pub direction: Direction
}

impl CreatureStack {
    pub fn new(creature: Creature, count: u32, position: GridPos, side: Side) -> Self {
        let direction = match side {
            Side::Attacker => Direction::Right,
            Side::Defender => Direction::Left
        };

        CreatureStack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            position,
            turn_state: CreatureTurnState::HasTurn,
            defending: false,
            draw_pos: position.draw_center(),
            animation_type: AnimationType::Standing,
            animation_progress: 0.0,

            current_animation: None,
            animation_queue: VecDeque::new(),

            direction
        }
    }

    // Логика

    pub fn base_stats(&self) -> CreatureStats {
        self.creature.base_stats()
    }

    pub fn speed(&self) -> u8 {
        self.base_stats().speed
    }

    pub fn is_alive(&self) -> bool {
        self.count == 0
    }

    pub fn get_occupied_cells(&self, side: Side) -> Vec<GridPos> {
        if self.creature.is_wide() {
            let second_cell =
                match side {
                    Side::Attacker => self.position.relative(0, 1),
                    Side::Defender => self.position.relative(0, -1)
                };
            vec![self.position, second_cell]
        } else {
            vec![self.position]
        }
    }

    pub fn get_adjacent_cells(&self, side: Side) -> Vec<GridPos> {
        self.get_occupied_cells(side)
            .iter()
            .map(|cell| cell.get_successors())
            .flatten()
            .collect::<HashSet<GridPos>>() // Оставляем уникальные
            .drain()
            .collect::<Vec<GridPos>>()
    }

    pub fn receive_damage(&mut self, damage: u32) {
        let unit_health = self.base_stats().health;
        let total_health = (self.count - 1) * unit_health as u32 + self.current_health as u32;
        if total_health <= damage {
            self.current_health = 0;
            self.count = 0;
        } else {
            let health_left = total_health - damage;
            let creatures_left = health_left / unit_health as u32;
            let current_health = health_left as u16 % unit_health;

            if current_health == 0 {
                self.count = creatures_left - 1;
                self.current_health = unit_health;
            } else {
                self.count = creatures_left;
                self.current_health = current_health;
            }
        }
    }

    // Графика

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
                if let Some((start_pos, end_pos)) = animation.tween_data() {
                    let diff = end_pos - start_pos;
                    let (diff_x, diff_y) = (diff.x(), diff.y());
                    let offset_x = (diff_x as f32 * progress) as i32;
                    let offset_y = (diff_y as f32 * progress) as i32;
                    self.draw_pos = start_pos.offset(offset_x, offset_y);
                }
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

        let draw_rect = sprite.draw_rect(self.draw_pos, self.direction);
        let texture = sprite.surface().as_texture(tc)?;

        match self.direction {
            Direction::Left =>
                canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?,
            Direction::Right =>
                canvas.copy(&texture, None, draw_rect)?
        };

        // canvas.set_draw_color(Color::RED);
        // canvas.draw_rect(draw_rect)?;
        // canvas.set_draw_color(Color::BLACK);

        Ok(())
    }

    pub fn push_animation(&mut self, animation: CreatureAnimation) {
        self.animation_queue.push_back(animation);
    }

    pub fn get_animations_duration(&self) -> Duration {
        let queue_duration =
            self.animation_queue
                .iter()
                .map(|animation| animation.total_duration())
                .sum();
       
        self.current_animation
            .as_ref()
            .map_or(queue_duration, |animation| {
                queue_duration + animation.total_duration()
            })
    }
}

use std::fmt;
impl fmt::Display for CreatureStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.creature, self.count)
    }
}
