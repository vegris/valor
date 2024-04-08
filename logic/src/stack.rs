use std::collections::HashSet;

use gamedata::creatures;
use gamedata::creatures::Creature;

use crate::grid::GridPos;
use crate::turn::Phase;

use super::gamestate::{GameState, Side};
use super::pathfinding;

#[derive(Clone, Debug)]
pub struct Stack {
    pub creature: Creature,
    pub count: i32,

    pub current_health: i32,
    pub current_ammo: i32,

    pub head: GridPos,
    pub side: Side,

    pub turn_state: Option<Phase>,
    pub defending: bool,

    pub retaliation_count: RetaliationCount,
}

#[derive(Clone, Debug)]
pub enum RetaliationCount {
    Finite(i32),
    Infinite,
}

impl RetaliationCount {
    pub fn from_creature(creature: Creature) -> Self {
        creature
            .abilities()
            .into_iter()
            .find_map(|ability| match ability {
                creatures::Ability::ExtraRetaliation => Some(Self::Finite(2)),
                creatures::Ability::InfiniteRetaliations => Some(Self::Infinite),
                _ => None,
            })
            .unwrap_or(Self::Finite(1))
    }

    pub fn has_retaliation(&self) -> bool {
        !matches!(self, RetaliationCount::Finite(0))
    }

    pub fn decrement(&mut self) {
        if let RetaliationCount::Finite(n) = self {
            assert!(*n != 0);
            *n -= 1;
        }
    }
}

impl Stack {
    pub fn new(creature: Creature, count: i32, head: GridPos, side: Side) -> Self {
        Stack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            head,
            side,
            turn_state: Some(Phase::Fresh),
            defending: false,
            retaliation_count: RetaliationCount::from_creature(creature),
        }
    }

    pub fn refresh_for_next_turn(&mut self) {
        self.turn_state = Some(Phase::Fresh);
        self.retaliation_count = RetaliationCount::from_creature(self.creature);
    }

    pub fn base_stats(&self) -> creatures::Stats {
        self.creature.base_stats()
    }

    pub fn speed(&self) -> i32 {
        self.base_stats().speed
    }

    pub fn can_shoot(&self, state: &GameState) -> bool {
        let has_ammo = self.current_ammo != 0;
        let has_enemies_around = self
            .get_adjacent_cells()
            .iter()
            .filter_map(|&cell| state.find_unit_for_cell(cell))
            .any(|handle| state.get_stack(handle).side != self.side);
        has_ammo && !has_enemies_around
    }

    pub fn is_alive(&self) -> bool {
        self.count > 0
    }

    pub fn get_occupied_cells(&self) -> Vec<GridPos> {
        pathfinding::get_occupied_cells_for(self.creature, self.side, self.head).unwrap()
    }

    fn get_adjacent_cells(&self) -> Vec<GridPos> {
        self.get_occupied_cells()
            .iter()
            .flat_map(|cell| cell.get_successors())
            .collect::<HashSet<GridPos>>() // Оставляем уникальные
            .drain()
            .collect::<Vec<GridPos>>()
    }

    pub fn receive_damage(&mut self, damage: i32) {
        let creature_health = self.creature.base_stats().health;

        let total_health = (self.count - 1) * creature_health + self.current_health - damage;

        if total_health <= 0 {
            self.current_health = 0;
            self.count = 0;
            return;
        }

        let div = total_health / creature_health;
        let rem = total_health % creature_health;
        if rem == 0 {
            self.count = div;
            self.current_health = creature_health;
        } else {
            self.count = div + 1;
            self.current_health = rem;
        }
    }
}

use std::fmt;
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.creature, self.count)
    }
}
