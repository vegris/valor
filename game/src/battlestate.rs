use std::collections::HashMap;
use std::error::Error;

use strum_macros::EnumIter;

use crate::command::Command;
use crate::config::Config;
use crate::grid::GridPos;
use crate::stack::Stack;

use crate::pathfinding::NavigationArray;

mod army;
mod commands;
mod hero;
pub mod turns;

use hero::Hero;

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
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
enum Winner {
    Side(Side),
    Tie,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CreatureStackHandle(u32);

pub struct Stacks(HashMap<CreatureStackHandle, Stack>);

impl Stacks {
    fn get_many_mut<const N: usize>(
        &mut self,
        handles: [CreatureStackHandle; N],
    ) -> Option<[&mut Stack; N]> {
        use std::mem::MaybeUninit;

        for index in 1..N {
            if handles[index] == handles[index - 1] {
                return None;
            }
        }

        let mut arr: MaybeUninit<[&mut Stack; N]> = MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        // SAFETY: We expect `handles` to contain disjunct values that are in bounds of `self`.
        unsafe {
            for (i, handle) in handles.iter().enumerate() {
                if let Some(stack) = self.0.get_mut(handle) {
                    *(*arr_ptr).get_unchecked_mut(i) = &mut *(stack as *mut _);
                } else {
                    return None;
                }
            }

            Some(arr.assume_init())
        }
    }
}

pub struct BattleState {
    // Логика
    heroes: [Option<Hero>; 2],
    stacks: Stacks,
    turn: turns::Turn,
    current_stack: CreatureStackHandle,

    // Поиск пути
    navigation_array: NavigationArray,
    reachable_cells: Vec<GridPos>,
}

impl BattleState {
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let attacker_army = army::form_units(&config.armies[0].stacks, Side::Attacker);
        let defender_army = army::form_units(&config.armies[1].stacks, Side::Defender);

        let heroes = config.armies.map(|army| army.hero.map(Hero::build));

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
            heroes,
            stacks: Stacks(stacks),
            turn: turns::Turn::new(),
            current_stack: CreatureStackHandle(0),
            navigation_array: NavigationArray::empty(),
            reachable_cells: vec![],
        };

        state.update_current_stack();
        Ok(state)
    }

    pub fn is_command_applicable(&self, command: Command) -> bool {
        commands::is_applicable(self, command)
    }

    pub fn apply_command(&mut self, command: Command) {
        assert!(commands::is_applicable(self, command));
        commands::apply(self, command);
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
    }

    pub fn get_stack(&self, handle: CreatureStackHandle) -> &Stack {
        &self.stacks.0[&handle]
    }

    fn get_stack_mut(&mut self, handle: CreatureStackHandle) -> &mut Stack {
        self.stacks.0.get_mut(&handle).unwrap()
    }

    pub fn is_current(&self, handle: CreatureStackHandle) -> bool {
        self.current_stack == handle
    }

    pub fn get_current_stack(&self) -> &Stack {
        self.get_stack(self.current_stack)
    }

    fn get_current_stack_mut(&mut self) -> &mut Stack {
        self.get_stack_mut(self.current_stack)
    }

    pub fn units(&self) -> Vec<CreatureStackHandle> {
        self.stacks.0.keys().copied().collect()
    }

    pub fn find_unit_for_cell(&self, cell: GridPos) -> Option<CreatureStackHandle> {
        self.units()
            .into_iter()
            .filter(|&handle| self.get_stack(handle).is_alive())
            .find(|&handle| self.get_stack(handle).get_occupied_cells().contains(&cell))
    }

    pub fn reachable_cells(&self) -> &Vec<GridPos> {
        &self.reachable_cells
    }

    fn update_current_stack(&mut self) {
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

                for stack in self.stacks.0.values_mut() {
                    stack.turn_state = Some(turns::Phase::Fresh);
                    stack.retaliation_count = stack.creature.retaliation_count();
                    stack.defending = false;
                }
            }
            self.update_current_stack();
        }
    }

    fn find_winner(&self) -> Option<Winner> {
        let alive_sides = [Side::Attacker, Side::Defender]
            .into_iter()
            .filter(|&side| {
                self.stacks
                    .0
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
