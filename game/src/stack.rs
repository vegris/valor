use std::collections::HashSet;

use gamedata::{Creature, CreatureStats, RetaliationCount};

use crate::battlestate::turns;
use crate::grid::GridPos;

use super::battlestate::{BattleState, Side};
use super::pathfinding;

#[derive(Clone, Debug)]
pub struct Stack {
    pub creature: Creature,
    pub count: u32,

    pub current_health: u16,
    pub current_ammo: u8,

    pub head: GridPos,
    pub side: Side,

    pub turn_state: Option<turns::Phase>,
    pub defending: bool,

    pub retaliation_count: RetaliationCount,
}

impl Stack {
    pub fn new(creature: Creature, count: u32, head: GridPos, side: Side) -> Self {
        Stack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            head,
            side,
            turn_state: Some(turns::Phase::Fresh),
            defending: false,
            retaliation_count: creature.retaliation_count(),
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

    pub fn get_adjacent_cells(&self) -> Vec<GridPos> {
        self.get_occupied_cells()
            .iter()
            .flat_map(|cell| cell.get_successors())
            .collect::<HashSet<GridPos>>() // Оставляем уникальные
            .drain()
            .collect::<Vec<GridPos>>()
    }
}

use std::fmt;
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.creature, self.count)
    }
}
