use super::hero::Hero;
use super::creature::Creature;
use super::creature_stack::{CreatureStack, CreatureTurnState};
use super::command::{Command, CommandType};

#[derive(PartialEq)]
pub enum StrikeType {
    Melee,
    Ranged
}

#[derive(Clone, Copy, PartialEq)]
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

pub struct BattleState {
    sides: [Army; 2],
    current_phase: CreatureTurnState,
    current_side: Side,
    current_stack: usize,
    is_moraled: bool
}

impl BattleState {
    pub fn new(attacker_army: Army, defender_army: Army) -> Self {
        Self {
            sides: [attacker_army, defender_army],
            current_phase: CreatureTurnState::HasTurn,
            current_side: Side::Attacker,
            current_stack: 0,
            is_moraled: false
        }
    }
    pub fn is_applicable(&self, command: Command) -> bool {
        true
    }
    pub fn apply(&mut self, command: Command) {

    }
}