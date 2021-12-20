use std::collections::HashSet;
use std::error::Error;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use gamedata::{Creature, CreatureStats};
use gridpos::GridPos;

use crate::registry::ResourceRegistry;
use crate::graphics::creature::AnimationType;

use super::battlestate::{BattleState, Side};
use super::pathfinding;

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

#[derive(Clone)]
pub struct CreatureStack {
    pub creature: Creature,
    pub count: u32,

    pub current_health: u16,
    pub current_ammo: u8,

    pub head: GridPos,

    pub turn_state: CreatureTurnState,
    pub defending: bool,

    pub direction: Direction
}

impl CreatureStack {
    pub fn new(creature: Creature, count: u32, head: GridPos, side: Side) -> Self {
        let direction = match side {
            Side::Attacker => Direction::Right,
            Side::Defender => Direction::Left
        };

        CreatureStack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            head,
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

    pub fn can_shoot(&self, side: Side, state: &BattleState) -> bool {
        let has_ammo = self.current_ammo != 0;
        let has_enemies_around =
            self.get_adjacent_cells(side)
                .iter()
                .filter_map(|&cell| state.find_unit_for_cell(cell))
                .find(|stack| stack.side != side) 
                .is_some();
        has_ammo && !has_enemies_around
    }
    
    pub fn is_alive(&self) -> bool {
        self.count > 0
    }

    pub fn tail(&self, side: Side) -> GridPos {
        pathfinding::tail_for(self.creature, side, self.head)
            .unwrap()
    }
    
    pub fn get_occupied_cells(&self, side: Side) -> Vec<GridPos> {
        pathfinding::get_occupied_cells_for(self.creature, side, self.head)
            .unwrap()
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

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>,
        is_selected: bool,
        side: Side,
        stack_count_bg: &Texture,
        font: &Font
    ) -> Result<(), Box<dyn Error>> {
        let spritesheet = rr.get_creature_container(self.creature);

        let animation_type =
            if self.is_alive() {
                AnimationType::Standing
            } else {
                AnimationType::Death
            };

        let animation_block = spritesheet.animation_block(animation_type);
        let sprite_index = animation_block[animation_block.len() - 1];
        let sprite = &mut spritesheet.sprites[sprite_index];
        if is_selected { sprite.turn_selection(&mut spritesheet.colors, true) };

        let draw_rect = sprite.draw_rect(self.tail(side).center(), self.direction);
        let texture = sprite.surface().as_texture(tc)?;

        match self.direction {
            Direction::Left =>
                canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?,
            Direction::Right =>
                canvas.copy(&texture, None, draw_rect)?
        };

        if is_selected { sprite.turn_selection(&mut spritesheet.colors, false) };

        if self.is_alive() {
            let cell_center = self.head.bounding_rect().center();
            let draw_center = cell_center.offset(0, 10);
            canvas.copy(stack_count_bg, None, Rect::from_center(draw_center, 30, 11))?;

            let font_surface = font.render(&self.count.to_string()).solid(Color::BLUE)?;
            let font_texture = font_surface.as_texture(&tc)?;

            let mut font_rect = font_surface.rect();
            font_rect.center_on(draw_center);

            canvas.copy(&font_texture, None, font_rect)?;
        }

        Ok(())
    }

}

use std::fmt;
impl fmt::Display for CreatureStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.creature, self.count)
    }
}
