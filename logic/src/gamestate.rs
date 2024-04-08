use std::collections::HashMap;

use gamedata::creatures::Creature;
use gamedata::heroes::Hero as GDHero;
use serde::Deserialize;
use strum_macros::EnumIter;

use common::error::AnyHow;

use crate::command::Command;
use crate::event::Event;
use crate::grid::GridPos;
use crate::pathfinding::NavigationArray;
use crate::stack::Stack;
use crate::turn::Turn;

mod army;
mod commands;
mod damage;
mod hero;

use hero::Hero;

pub struct GameState {
    // Логика
    heroes: [Option<Hero>; 2],
    stacks: HashMap<StackHandle, Stack>,
    turn: Turn,
    current_stack: StackHandle,

    // Поиск пути
    navigation_array: NavigationArray,
    reachable_cells: Vec<GridPos>,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Army {
    pub hero: Option<GDHero>,
    pub stacks: [Option<(Creature, i32)>; 7],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StackHandle(u32);

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
pub enum Side {
    Attacker,
    Defender,
}

#[derive(Debug)]
enum Winner {
    Side(Side),
    Tie,
}

impl GameState {
    pub fn new(armies: &[Army; 2]) -> AnyHow<Self> {
        let attacker_army = army::form_units(&armies[0].stacks, Side::Attacker);
        let defender_army = army::form_units(&armies[1].stacks, Side::Defender);

        let heroes = armies.map(|army| army.hero.map(Hero::build));

        let stacks = [attacker_army, defender_army]
            .concat()
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let handle = StackHandle(i as u32);
                (handle, v)
            })
            .collect();

        let mut state = Self {
            heroes,
            stacks,
            turn: Turn::new(),
            current_stack: StackHandle(0),
            navigation_array: NavigationArray::empty(),
            reachable_cells: vec![],
        };

        state.update_current_stack();
        Ok(state)
    }

    pub fn is_command_applicable(&self, command: Command) -> bool {
        commands::is_applicable(self, command)
    }

    pub fn apply_command(&mut self, command: Command) -> Vec<Event> {
        assert!(commands::is_applicable(self, command));
        let events = commands::apply(self, command);
        println!("Command applied!");

        if command.spends_turn() {
            let cur_stack = self.get_current_stack_mut();
            cur_stack.turn_state = None;
        }

        if command.requires_current_stack_update() {
            self.update_current_stack();
        }

        if let Some(winner) = self.find_winner() {
            println!("{:?} wins!", winner);
            std::process::exit(0);
        }

        events
    }

    pub fn get_stack(&self, handle: StackHandle) -> &Stack {
        &self.stacks[&handle]
    }

    fn get_stack_mut(&mut self, handle: StackHandle) -> &mut Stack {
        self.stacks.get_mut(&handle).unwrap()
    }

    pub fn is_current(&self, handle: StackHandle) -> bool {
        self.current_stack == handle
    }

    pub fn get_current_stack(&self) -> &Stack {
        self.get_stack(self.current_stack)
    }

    fn get_current_stack_mut(&mut self) -> &mut Stack {
        self.get_stack_mut(self.current_stack)
    }

    pub fn units(&self) -> Vec<StackHandle> {
        self.stacks.keys().copied().collect()
    }

    pub fn find_unit_for_cell(&self, cell: GridPos) -> Option<StackHandle> {
        self.units()
            .into_iter()
            .filter(|&handle| self.get_stack(handle).is_alive())
            .find(|&handle| self.get_stack(handle).get_occupied_cells().contains(&cell))
    }

    pub fn reachable_cells(&self) -> &Vec<GridPos> {
        &self.reachable_cells
    }

    fn update_current_stack(&mut self) {
        if let Some(handle) = self.find_active_stack() {
            self.current_stack = handle;

            let stack = self.get_current_stack_mut();
            stack.defending = false;
            println!("Current stack is {}", stack);

            let stack_head = stack.head;
            let is_flying = stack.creature.is_flying();
            let stack_speed = stack.speed();

            let navigation_array = NavigationArray::new(stack_head, self, is_flying);
            let reachable_cells = navigation_array.get_reachable_cells(stack_speed);
            self.navigation_array = navigation_array;
            self.reachable_cells = reachable_cells;
        } else {
            if !self.turn.try_advance_phase() {
                self.turn = self.turn.next();

                for stack in self.stacks.values_mut() {
                    stack.refresh_for_next_turn();
                }
            }
            self.update_current_stack();
        }
    }
    fn find_active_stack(&self) -> Option<StackHandle> {
        let mut handles = self.units();
        // Преимущество при равенстве скоростей у того кто ходил вторым на прошлом ходу
        handles
            .sort_unstable_by_key(|&handle| self.get_stack(handle).side == self.turn.priority_side);

        handles
            .iter()
            .map(|&handle| (handle, self.get_stack(handle)))
            .filter(|(_handle, stack)| stack.is_alive())
            .filter(|(_handle, stack)| {
                stack
                    .turn_state
                    .map_or(false, |phase| phase == self.turn.current_phase)
            })
            .fold(None, |acc, current| {
                // Без max_first тяжко
                fn key((_, stack): (StackHandle, &Stack)) -> i32 {
                    stack.speed()
                }
                match acc {
                    None => Some(current),
                    Some(acc) if key(current) > key(acc) => Some(current),
                    _ => acc,
                }
            })
            .map(|(handle, _stack)| handle)
    }

    fn find_winner(&self) -> Option<Winner> {
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

impl Side {
    pub fn other(self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker,
        }
    }
}
