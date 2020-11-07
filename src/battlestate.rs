use super::hero::Hero;
use super::creature::Creature;
use super::creature_stack::{CreatureStack, CreatureTurnState};
use super::command::{Command, CommandType};

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

pub struct Army {
    hero: Hero,
    starting_army: [Option<(Creature, u32)>; 7],
    battle_army: Vec<CreatureStack>
}

impl Army {
    pub fn new(hero: Hero, starting_army: [Option<(Creature, u32)>; 7]) -> Self {
        let battle_army = starting_army
            .iter()
            .filter(|x| x.is_some())
            .map(|option| {
                let (creature, count) = option.unwrap();
                CreatureStack::new(creature, count)
            })
            .collect::<Vec<CreatureStack>>();

        Self {
            hero,
            starting_army,
            battle_army
        }
    }
}

type PhaseIterator = std::vec::IntoIter<CreatureTurnState>;

pub struct BattleState {
    sides: [Army; 2],
    phase_iter: PhaseIterator,
    current_phase: CreatureTurnState,
    current_side: Side,
    current_stack: usize,
    is_moraled: bool
}

impl BattleState {
    fn new_phase_iter() -> PhaseIterator {
        type CTS = CreatureTurnState;
        vec![CTS::HasTurn, CTS::MoraledAndWaited, CTS::Waited].into_iter()
    }
    pub fn new(attacker_army: Army, defender_army: Army) -> Self {
        Self {
            sides: [attacker_army, defender_army],
            current_phase: CreatureTurnState::HasTurn,
            phase_iter: Self::new_phase_iter(),
            current_side: Side::Attacker,
            current_stack: 0,
            is_moraled: false
        }
    }
    fn battle_army(&self, side: Side) -> &Vec<CreatureStack> {
        &self.sides[side as usize].battle_army
    }

    pub fn current_side(&self) -> Side {
        self.current_side
    }

    fn get_stack(&self, side: Side, index: usize) -> &CreatureStack {
        &self.sides[side as usize].battle_army[index]
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
            println!("Current stack is {:?}:{:?}, {:?}", stack.creature(), stack.count(), side);
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
            .for_each(|creature| creature.turn_state = CreatureTurnState::HasTurn);
        println!("New turn!");
    }

    fn find_current_creature(&self) -> Option<(Side, usize)> {
        // Преимущество при равенстве скоростей у того кто ходил вторым на прошлом ходу
        match self.current_side {
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
        .fold(None, |acc, (side, index, stack)| {
            match acc {
                None => Some((side, index, stack)),
                Some((_, _, acc_stack)) if stack.speed() > acc_stack.speed() => Some((side, index, stack)),
                _ => acc
            }
        })
        .map(|(side, index, stack)| (side, index))
    }
}