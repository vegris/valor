use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use strum_macros::EnumIter;

use crate::command::Command;
use crate::config::Config;
use crate::graphics::animation::{Anim, AtEndEvent};
use crate::graphics::spritesheet::creature::AnimationType;
use crate::grid::GridPos;
use crate::registry::ResourceRegistry;
use crate::stack::Stack;

use crate::pathfinding::NavigationArray;

mod army;
mod commands;
mod damage;
mod hero;
pub mod turns;

use hero::Hero;

use self::commands::Event;

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
pub struct StackHandle(u32);

pub struct Stacks(HashMap<StackHandle, Stack>);

impl Stacks {
    fn get_many_mut<const N: usize>(
        &mut self,
        handles: [StackHandle; N],
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
    current_stack: StackHandle,

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
                let handle = StackHandle(i as u32);
                (handle, v)
            })
            .collect();

        let mut state = Self {
            heroes,
            stacks: Stacks(stacks),
            turn: turns::Turn::new(),
            current_stack: StackHandle(0),
            navigation_array: NavigationArray::empty(),
            reachable_cells: vec![],
        };

        state.update_current_stack();
        Ok(state)
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        for stack in self.stacks.0.values_mut() {
            stack.update(dt, rr);
        }
    }

    pub fn is_command_applicable(&self, command: Command) -> bool {
        commands::is_applicable(self, command)
    }

    pub fn apply_command(&mut self, command: Command, rr: &mut ResourceRegistry) {
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

        process_events(self, events, rr);
    }

    pub fn get_stack(&self, handle: StackHandle) -> &Stack {
        &self.stacks.0[&handle]
    }

    fn get_stack_mut(&mut self, handle: StackHandle) -> &mut Stack {
        self.stacks.0.get_mut(&handle).unwrap()
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
        self.stacks.0.keys().copied().collect()
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

fn process_events(state: &mut BattleState, events: Vec<Event>, rr: &mut ResourceRegistry) {
    for event in events {
        match event {
            Event::Attack {
                attacker,
                defender,
                strikes,
            } => {
                let [attacker, defender] = state.stacks.get_many_mut([attacker, defender]).unwrap();

                let needs_turning = needs_turning(attacker, defender);

                if needs_turning {
                    let mut attacker_animations = vec![];
                    let mut defender_animations = vec![];

                    attacker_animations.push(
                        Anim::new(AnimationType::TurnLeft, attacker.creature, rr)
                            .set_at_end(AtEndEvent::InvertSide),
                    );

                    let anim = Anim::new(AnimationType::TurnRight, attacker.creature, rr);
                    attacker_animations.push(anim);

                    defender_animations.push(
                        Anim::new(AnimationType::TurnLeft, defender.creature, rr)
                            .set_at_end(AtEndEvent::InvertSide),
                    );

                    let anim = Anim::new(AnimationType::TurnRight, defender.creature, rr);
                    defender_animations.push(anim);

                    let attacker_duration = attacker.animation_queue.total_duration();
                    let defender_duration = defender.animation_queue.total_duration();
                    if attacker_duration > defender_duration {
                        defender_animations[0] =
                            defender_animations[0].add_delay(attacker_duration - defender_duration);
                    } else {
                        attacker_animations[0] =
                            attacker_animations[0].add_delay(defender_duration - attacker_duration);
                    }

                    for animation in attacker_animations {
                        attacker.animation_queue.push(animation);
                    }
                    for animation in defender_animations {
                        defender.animation_queue.push(animation);
                    }
                }

                for strike in strikes {
                    if strike.retaliation {
                        animate_strike(defender, attacker, strike.lethal, rr);
                    } else {
                        animate_strike(attacker, defender, strike.lethal, rr);
                    }
                }

                if needs_turning {
                    let mut attacker_animations = vec![];
                    let mut defender_animations = vec![];

                    if attacker.is_alive() {
                        let anim = Anim::new(AnimationType::TurnLeft, attacker.creature, rr)
                            .set_at_end(AtEndEvent::InvertSide);
                        attacker_animations.push(anim);

                        let anim = Anim::new(AnimationType::TurnRight, attacker.creature, rr);
                        attacker_animations.push(anim);
                    }

                    if defender.is_alive() {
                        let anim = Anim::new(AnimationType::TurnLeft, defender.creature, rr)
                            .set_at_end(AtEndEvent::InvertSide);
                        defender_animations.push(anim);

                        let anim = Anim::new(AnimationType::TurnRight, defender.creature, rr);
                        defender_animations.push(anim);
                    }

                    let attacker_duration = attacker.animation_queue.total_duration();
                    let defender_duration = defender.animation_queue.total_duration();
                    if attacker_duration > defender_duration {
                        defender_animations[0] =
                            defender_animations[0].add_delay(attacker_duration - defender_duration);
                    } else {
                        attacker_animations[0] =
                            attacker_animations[0].add_delay(defender_duration - attacker_duration);
                    }

                    for animation in attacker_animations {
                        attacker.animation_queue.push(animation);
                    }
                    for animation in defender_animations {
                        defender.animation_queue.push(animation);
                    }
                }
            }
            Event::Movement { stack_handle, path } => {
                if path.len() == 1 {
                    continue;
                }

                let stack = state.get_stack_mut(stack_handle);

                let mut cur_side = stack.side;
                let mut animations = vec![];

                let start = path[0].center();
                let animation = Anim::new(AnimationType::StartMoving, stack.creature, rr)
                    .add_tween(start, start);
                animations.push(animation);

                let mut from = path[0];

                if facing_side(path[0], path[1]) != stack.side {
                    let anim = Anim::new(AnimationType::TurnLeft, stack.creature, rr)
                        .set_at_end(AtEndEvent::InvertSide)
                        .add_tween(from.center(), from.center());
                    animations.push(anim);
                    let anim = Anim::new(AnimationType::TurnRight, stack.creature, rr)
                        .add_tween(from.center(), from.center());
                    animations.push(anim);

                    cur_side = cur_side.other();
                }

                if !stack.creature.is_teleporting() {
                    let animation = Anim::new(AnimationType::Moving, stack.creature, rr)
                        .add_tween(from.center(), path[1].center());
                    animations.push(animation);

                    from = path[1];

                    for to in &path[2..] {
                        let animation = Anim::new(AnimationType::Moving, stack.creature, rr)
                            .add_tween(from.center(), to.center());

                        if facing_side(from, *to) != cur_side {
                            if let Some(animation) = animations.last_mut() {
                                *animation = animation.set_at_end(AtEndEvent::InvertSide);
                            }
                            cur_side = cur_side.other();
                        }

                        animations.push(animation);
                        from = *to;
                    }
                }

                let animation = Anim::new(AnimationType::StopMoving, stack.creature, rr);
                animations.push(animation);

                if cur_side != stack.side {
                    let anim = Anim::new(AnimationType::TurnLeft, stack.creature, rr)
                        .set_at_end(AtEndEvent::InvertSide);
                    animations.push(anim);
                    let anim = Anim::new(AnimationType::TurnRight, stack.creature, rr);
                    animations.push(anim);
                }

                for animation in animations {
                    stack.animation_queue.push(animation);
                }
            }
            Event::Shot {
                attacker,
                target,
                lethal,
            } => {
                let [attacker, target] = state.stacks.get_many_mut([attacker, target]).unwrap();
                let mut animation = Anim::new(AnimationType::ShootStraight, attacker.creature, rr);
                let shoot_duration = animation.duration();

                let attacker_duration = attacker.animation_queue.total_duration();
                let target_duration = target.animation_queue.total_duration();

                if target_duration > attacker_duration {
                    let delay = target_duration - attacker_duration;
                    animation = animation.add_delay(delay);
                }
                attacker.animation_queue.push(animation);

                let animation_type = if lethal {
                    AnimationType::Death
                } else if target.defending {
                    AnimationType::Defend
                } else {
                    AnimationType::GettingHit
                };

                let animation =
                    Anim::new(animation_type, target.creature, rr).add_delay(shoot_duration);
                target.animation_queue.push(animation);
            }
        }
    }
}

fn animate_strike(
    attacker: &mut Stack,
    defender: &mut Stack,
    lethal: bool,
    rr: &mut ResourceRegistry,
) {
    let animation = Anim::new(AnimationType::AttackStraight, attacker.creature, rr);

    let total_delay = attacker
        .animation_queue
        .total_duration()
        .checked_sub(defender.animation_queue.total_duration())
        .unwrap_or(Duration::ZERO)
        + animation.duration() / 2;
    attacker.animation_queue.push(animation);

    let animation_type = if lethal {
        AnimationType::Death
    } else if defender.defending {
        AnimationType::Defend
    } else {
        AnimationType::GettingHit
    };
    let animation = Anim::new(animation_type, defender.creature, rr).add_delay(total_delay);
    defender.animation_queue.push(animation);
}

fn facing_side(pos: GridPos, target: GridPos) -> Side {
    assert!(pos != target);

    if pos.y == target.y {
        if pos.x > target.x {
            Side::Defender
        } else {
            Side::Attacker
        }
    } else if pos.is_even_row() {
        if pos.x <= target.x {
            Side::Attacker
        } else {
            Side::Defender
        }
    } else if pos.x >= target.x {
        Side::Defender
    } else {
        Side::Attacker
    }
}

fn needs_turning(attacker: &Stack, defender: &Stack) -> bool {
    facing_side(attacker.head, defender.head) != attacker.side
}
