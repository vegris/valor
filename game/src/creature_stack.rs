use std::collections::HashSet;
use std::time::Duration;
use std::error::Error;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Point;

use creature::{Creature, CreatureStats};

use crate::registry::ResourceRegistry;
use crate::graphics::creature::{CreatureSprite, AnimationType};

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
    pub creature: Creature,
    pub count: u32,

    pub current_health: u16,
    pub current_ammo: u8,

    pub position: GridPos,

    pub turn_state: CreatureTurnState,
    pub defending: bool,

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
            direction
        }
    }

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
                    Side::Attacker => self.position.relative(1, 0),
                    Side::Defender => self.position.relative(-1, 0)
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

    pub fn update(&mut self, _dt: Duration) {}

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>,
        is_selected: bool
    ) -> Result<(), Box<dyn Error>> {
        let mut spritesheet = rr.get_creature_container(self.creature);
        let animation_block = spritesheet.animation_block(AnimationType::Standing);
        let sprite_index = animation_block[0];
        let mut sprite = &mut spritesheet.sprites[sprite_index];
        if is_selected { sprite.turn_selection(&mut spritesheet.colors, true) };

        let draw_rect = sprite.draw_rect(self.position.center(), self.direction);
        let texture = sprite.surface().as_texture(tc)?;

        match self.direction {
            Direction::Left =>
                canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?,
            Direction::Right =>
                canvas.copy(&texture, None, draw_rect)?
        };

        if is_selected { sprite.turn_selection(&mut spritesheet.colors, false) };

        Ok(())
    }

}

use std::fmt;
impl fmt::Display for CreatureStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.creature, self.count)
    }
}
