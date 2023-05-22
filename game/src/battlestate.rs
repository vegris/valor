use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use gridpos::GridPos;

use crate::animations::Animation;
use crate::config::Config;
use crate::creature_stack::CreatureStack;
use crate::graphics::creature::AnimationType;
use crate::pathfinding::NavigationArray;
use crate::registry::ResourceRegistry;

mod army;
pub mod input;
pub mod turns;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
    Attacker,
    Defender,
}

impl Side {
    pub fn other(self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker,
        }
    }
}

#[derive(Debug)]
pub enum Winner {
    Side(Side),
    Tie,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CreatureStackHandle(u32);

pub struct BattleState {
    // Логика
    pub stacks: HashMap<CreatureStackHandle, CreatureStack>,
    pub turn: turns::Turn,
    pub current_stack: CreatureStackHandle,

    // Поиск пути
    pub navigation_array: NavigationArray,
    pub reachable_cells: Vec<GridPos>,

    previous_mouseover_stack: Option<CreatureStackHandle>,
}

impl BattleState {
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let attacker_army = army::form_units(&config.armies[0], Side::Attacker);
        let defender_army = army::form_units(&config.armies[1], Side::Defender);

        let stacks = [attacker_army, defender_army]
            .concat()
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let handle = CreatureStackHandle(i as u32);
                (handle, v)
            })
            .collect();

        let mut state = Self {
            stacks,
            turn: turns::Turn::new(),
            current_stack: CreatureStackHandle(0),
            navigation_array: NavigationArray::empty(),
            reachable_cells: vec![],

            previous_mouseover_stack: None,
        };

        let animation = Animation::new(AnimationType::Standing);
        for stack in state.stacks.values_mut() {
            stack.animation_queue.add(animation);
        }

        state.update_current_stack();
        Ok(state)
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        for stack in self.stacks.values_mut() {
            stack.update(dt, rr);
        }
    }

    pub fn update_current_stack(&mut self) {
        if let Some(handle) = turns::find_active_stack(self) {
            self.current_stack = handle;

            let mut stack = self.get_current_stack_mut();
            stack.defending = false;
            println!("Current stack is {}", stack);

            let stack_head = stack.head;
            let is_flying = stack.creature.is_flying();
            let stack_speed = stack.speed().into();

            let navigation_array = NavigationArray::new(stack_head, self, is_flying);
            let reachable_cells = navigation_array.get_reachable_cells(stack_speed);
            self.navigation_array = navigation_array;
            self.reachable_cells = reachable_cells;
        } else {
            if !self.turn.try_advance_phase() {
                self.turn = self.turn.next();

                for stack in self.stacks.values_mut() {
                    stack.turn_state = Some(turns::Phase::Fresh)
                }
            }
            self.update_current_stack();
        }
    }

    pub fn get_stack(&self, handle: CreatureStackHandle) -> &CreatureStack {
        &self.stacks[&handle]
    }
    pub fn get_stack_mut(&mut self, handle: CreatureStackHandle) -> &mut CreatureStack {
        self.stacks.get_mut(&handle).unwrap()
    }

    pub fn get_current_stack(&self) -> &CreatureStack {
        self.get_stack(self.current_stack)
    }

    pub fn get_current_stack_mut(&mut self) -> &mut CreatureStack {
        self.get_stack_mut(self.current_stack)
    }

    pub fn units(&self) -> Vec<CreatureStackHandle> {
        self.stacks.keys().copied().collect()
    }

    pub fn find_unit_for_cell(&self, cell: GridPos) -> Option<CreatureStackHandle> {
        self.units()
            .into_iter()
            .filter(|&handle| self.get_stack(handle).is_alive())
            .find(|&handle| self.get_stack(handle).get_occupied_cells().contains(&cell))
    }

    pub fn find_winner(&self) -> Option<Winner> {
        let alive_sides = [Side::Attacker, Side::Defender]
            .into_iter()
            .filter(|&side| {
                self.stacks
                    .values()
                    .filter(|stack| stack.side == side)
                    .any(|stack| stack.is_alive())
            })
            .collect::<Vec<Side>>();

        match alive_sides.len() {
            0 => Some(Winner::Tie),
            1 => Some(Winner::Side(alive_sides[0])),
            2 => None,
            _ => unreachable!(),
        }
    }
}
