use std::cmp::{Ord, Ordering};

use super::hero::Hero;
use super::creature::Creature;
use super::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use super::command::{Command, CommandType};
use super::GridPos;

#[derive(PartialEq)]
pub enum StrikeType {
    Melee,
    Ranged
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
    Attacker,
    Defender
}
impl Side {
    pub fn other(self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker
        }
    }
}

pub struct Army {
    hero: Option<Hero>,
    starting_army: [Option<(Creature, u32)>; 7],
    battle_army: Vec<CreatureStack>
}

impl Army {
    pub fn new(
        hero: Option<Hero>,
        starting_army: [Option<(Creature, u32)>; 7],
        battle_army: Vec<CreatureStack>
    ) -> Self {
        Self {
            hero,
            starting_army,
            battle_army
        }
    }

    pub fn hero(&self) -> Option<&Hero> {
        self.hero.as_ref()
    }
}

type PhaseIterator = std::vec::IntoIter<CTS>;

pub struct BattleState {
    sides: [Army; 2],
    phase_iter: PhaseIterator,
    current_phase: CTS,
    last_turn_side: Side,
    current_side: Side,
    current_stack: usize,
    is_moraled: bool
}

fn initial_placement(units_count: u8) -> Vec<u16> {
    match units_count {
        1 => vec![6],
        2 => vec![3, 9],
        3 => vec![3, 6, 9],
        4 => vec![1, 5, 7, 11],
        5 => vec![1, 3, 6, 9, 11],
        6 => vec![1, 3, 5, 7, 9, 11],
        7 => vec![1, 3, 5, 6, 7, 9, 11],
        _ => unreachable!()
    }
}

fn form_units(starting_army: &[Option<(Creature, u32)>; 7], side: Side) -> Vec<CreatureStack> {
    let units_count = starting_army.iter().filter(|c| c.is_some()).count();
    let formation = initial_placement(units_count as u8);
    let starting_x = *match side {
        Side::Attacker => GridPos::X_RANGE.start(),
        Side::Defender => GridPos::X_RANGE.end()
    };
    starting_army
        .into_iter()
        .filter_map(|c| *c)
        .zip(formation.into_iter())
        .map(|((creature, count), y_pos)| {
            CreatureStack::new(creature, count, GridPos::new(starting_x, y_pos))
        })
        .collect()
}

impl BattleState {
    fn new_phase_iter() -> PhaseIterator {
        vec![CTS::HasTurn, CTS::Waited].into_iter()
    }
    pub fn new(
        attacker_hero: Option<Hero>,
        attacker_units: [Option<(Creature, u32)>; 7],
        defender_hero: Option<Hero>,
        defender_units: [Option<(Creature, u32)>; 7],
    ) -> Self {
        let attacker_stacks = form_units(&attacker_units, Side::Attacker);
        let defender_stacks = form_units(&defender_units, Side::Defender);

        let attacker_army = Army::new(attacker_hero, attacker_units, attacker_stacks);
        let defender_army = Army::new(defender_hero, defender_units, defender_stacks);

        let mut state = Self {
            sides: [attacker_army, defender_army],
            phase_iter: Self::new_phase_iter(),
            current_phase: CTS::HasTurn,
            last_turn_side: Side::Defender,
            current_side: Side::Attacker,
            current_stack: 0,
            is_moraled: false
        };

        state.update_current_stack();
        state
    }

    pub fn get_army(&self, side: Side) -> &Army {
        &self.sides[side as usize]
    }

    pub fn battle_army(&self, side: Side) -> &Vec<CreatureStack> {
        &self.sides[side as usize].battle_army
    }

    pub fn current_side(&self) -> Side {
        self.current_side
    }

    pub fn get_stack(&self, side: Side, index: u8) -> Option<&CreatureStack> {
        self.sides[side as usize].battle_army.get(index as usize)
    }
    pub fn get_stack_mut(&mut self, side: Side, index: u8) -> Option<&mut CreatureStack> {
        self.sides[side as usize].battle_army.get_mut(index as usize)
    }

    pub fn current_stack_id(&self) -> (Side, u8) {
        (self.current_side, self.current_stack as u8)
    }
    pub fn get_current_stack(&self) -> &CreatureStack {
        &self.sides[self.current_side as usize].battle_army[self.current_stack]
    }
    pub fn get_current_stack_mut(&mut self) -> &mut CreatureStack {
        &mut self.sides[self.current_side as usize].battle_army[self.current_stack]
    }

    pub fn update_current_stack(&mut self) {
        if let Some((side, index)) = self.find_current_creature() {
            self.current_side = side;
            self.current_stack = index;
            let mut stack = self.get_current_stack_mut();
            stack.defending = false;
            println!("Current stack is {}, {:?}", stack, side);
        } else {
            self.advance_phase();
            self.update_current_stack();
        }
    }

    pub fn advance_phase(&mut self) {
        if let Some(phase) = self.phase_iter.next() {
            self.current_phase = phase;
            println!("New turn phase: {:?}!", self.current_phase);
        } else {
            self.new_turn();
            self.advance_phase();
        }
    }

    pub fn new_turn(&mut self) {
        self.phase_iter = Self::new_phase_iter();
        self.sides
            .iter_mut()
            .map(|side| &mut side.battle_army)
            .flatten()
            .for_each(|creature| creature.turn_state = CTS::HasTurn);
        self.last_turn_side = self.last_turn_side.other();
        println!("New turn!");
    }

    fn find_current_creature(&self) -> Option<(Side, usize)> {
        // Преимущество при равенстве скоростей у того кто ходил вторым на прошлом ходу
        match self.last_turn_side {
            Side::Attacker => vec![Side::Defender, Side::Attacker],
            Side::Defender => vec![Side::Attacker, Side::Defender]
        }
        .into_iter()
        .flat_map(|side| Iterator::zip(
            std::iter::repeat(side),
            self.battle_army(side).iter().enumerate()
        ))
        .map(|(side, (index, stack))| (side, index, stack)) // чтоб не утонуть в скобках
        .filter(|(_side, index, stack)| stack.turn_state == self.current_phase)
        .fold(None, |acc, current| {
            // Без max_first тяжко
            fn key((_, _, stack): (Side, usize, &CreatureStack)) -> u8 {
                stack.speed()
            };
            match acc {
                None => Some(current),
                Some(acc) if key(current) > key(acc) => Some(current),
                _ => acc
            }
        })
        .map(|(side, index, stack)| (side, index))
    }
}
