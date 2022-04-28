use std::collections::HashSet;
use std::error::Error;
use std::time::Instant;

extern crate sdl2;
use sdl2::video::WindowContext;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use gamedata::{Creature, CreatureStats};
use gridpos::GridPos;

use crate::animations::{AnimationState, TweeningState};
use crate::registry::ResourceRegistry;
use crate::graphics::creature::AnimationType;

use super::battlestate::{BattleState, Side};
use super::pathfinding;
use super::animations::{Animation, Tweening};

/// Существо в течение раунда может принимать одно из этих состояний
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreatureTurnState {
    HasTurn,
    Waited,
    NoTurn
}

#[derive(Clone, Debug)]
pub struct CreatureStack {
    pub creature: Creature,
    pub count: u32,

    pub current_health: u16,
    pub current_ammo: u8,

    pub head: GridPos,
    pub side: Side,

    pub turn_state: CreatureTurnState,
    pub defending: bool,

    pub animation: Option<Animation>,
    pub tweening: Option<Tweening>
}

impl CreatureStack {
    pub fn new(creature: Creature, count: u32, head: GridPos, side: Side) -> Self {
        CreatureStack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            head,
            side,
            turn_state: CreatureTurnState::HasTurn,
            defending: false,
            animation: None,
            tweening: None
        }
    }

    pub fn base_stats(&self) -> CreatureStats {
        self.creature.base_stats()
    }

    pub fn speed(&self) -> u8 {
        self.base_stats().speed
    }

    pub fn can_shoot(&self, state: &BattleState) -> bool {
        let has_ammo = self.current_ammo != 0;
        let has_enemies_around =
            self.get_adjacent_cells()
                .iter()
                .filter_map(|&cell| state.find_unit_for_cell(cell))
                .any(|handle| state.get_stack(handle).side != self.side);
        has_ammo && !has_enemies_around
    }
    
    pub fn is_alive(&self) -> bool {
        self.count > 0
    }

    pub fn tail(&self) -> GridPos {
        pathfinding::tail_for(self.creature, self.side, self.head)
            .unwrap()
    }
    
    pub fn get_occupied_cells(&self) -> Vec<GridPos> {
        pathfinding::get_occupied_cells_for(self.creature, self.side, self.head)
            .unwrap()
    }

    pub fn get_adjacent_cells(&self) -> Vec<GridPos> {
        self.get_occupied_cells()
            .iter()
            .flat_map(|cell| cell.get_successors())
            .collect::<HashSet<GridPos>>() // Оставляем уникальные
            .drain()
            .collect::<Vec<GridPos>>()
    }

    pub fn update(&mut self, now: Instant) {
        if let Some(animation) = self.animation {
            if matches!(animation.state(now), AnimationState::Finished) {
                self.animation = Some(Animation{type_: AnimationType::Standing, start: now});
            }
        }
    }

    fn animation_index(&self, now: Instant, animation_len: usize) -> usize {
        self.animation
            .map(|animation| animation.state(now))
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

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>,
        is_selected: bool,
        stack_count_bg: &Texture,
        font: &Font,
        now: Instant
    ) -> Result<(), Box<dyn Error>> {
        let spritesheet = rr.get_creature_container(self.creature);

        let animation_type =
            if let Some(animation) = self.animation {
                animation.type_
            } else if self.is_alive() {
                AnimationType::Standing
            } else {
                AnimationType::Death
            };

        let animation_block = spritesheet.animation_block(animation_type);
        
        let animation_index = self.animation_index(now, animation_block.len());
        let sprite_index = animation_block[animation_index];
        let sprite = &mut spritesheet.sprites[sprite_index];
        if is_selected { sprite.turn_selection(&mut spritesheet.colors, true) };

        let draw_position =
            self.tweening
                .as_ref()
                .map(|tweening| tweening.state(now))
                .and_then(|state| {
                    if let TweeningState::Running(point) = state {
                        Some(point)
                    } else {
                        None
                    }
                })
                .unwrap_or(self.tail().center());

        let draw_rect = sprite.draw_rect(draw_position, self.side);
        let texture = sprite.surface().as_texture(tc)?;

        match self.side {
            Side::Attacker =>
                canvas.copy(&texture, None, draw_rect)?,
            Side::Defender =>
                canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?
        };

        if is_selected { sprite.turn_selection(&mut spritesheet.colors, false) };

        if self.is_alive() {
            let cell_center = self.head.bounding_rect().center();
            let draw_center = cell_center.offset(0, 10);
            canvas.copy(stack_count_bg, None, Rect::from_center(draw_center, 30, 11))?;

            let font_surface = font.render(&self.count.to_string()).solid(Color::BLUE)?;
            let font_texture = font_surface.as_texture(tc)?;

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
