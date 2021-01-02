#![allow(unused)]

#[macro_use]
extern crate num_derive;

mod creature;
mod creature_stack;
mod skills;
mod battlestate;
mod command;
mod hero;
mod functions;
mod gridpos;

use hero::{Hero, HeroAbility, HeroSpecialty};
use skills::SkillLevel;
use battlestate::{Army, Side, BattleState};
use creature::Creature;
use creature_stack::CreatureTurnState;
use command::{Command, CommandType};
use gridpos::GridPos;


fn main() {
    let attacker_hero = Hero {
        level: 5,

        attack: 1,
        defence: 1,
        spell_power: 1,
        knowledge: 1,
        specialty: HeroSpecialty::HeroAbility(HeroAbility::Offense),

        skills: vec![(HeroAbility::Offense, SkillLevel::Basic)],
        artifacts: vec![]
    };

    let defender_hero = Hero {
        level: 5,

        attack: 1,
        defence: 10,
        spell_power: 1,
        knowledge: 1,

        specialty: HeroSpecialty::HeroAbility(HeroAbility::Armorer),
        skills: vec![(HeroAbility::Armorer, SkillLevel::Expert)],
        artifacts: vec![]
    };

    let attacker_units = [
            Some((Creature::Angel, 8)),
            Some((Creature::Angel, 10)),
            None,
            None,
            Some((Creature::Archer, 55)),
            None,
            None
        ];

    let defender_units = [
            None,
            None,
            Some((Creature::Devil, 10)),
            Some((Creature::Angel, 20)),
            None,
            Some((Creature::HornedDemon, 25)),
            None
        ];

    let mut battlestate = BattleState::new(
        Some(attacker_hero), attacker_units,
        Some(defender_hero), defender_units
    );

    let commands = vec![
        Command::new(Side::Attacker, CommandType::Defend),
        Command::new(Side::Attacker, CommandType::Wait),
        Command::new(Side::Defender, CommandType::Move { destination: GridPos::new(4, 7) }),
        Command::new(Side::Defender, CommandType::Defend),
        Command::new(Side::Defender, CommandType::Move { destination: GridPos::new(11, 8)}),
        Command::new(Side::Attacker, CommandType::Shoot { target: 2 }),
        Command::new(Side::Attacker, CommandType::Defend),
    ];

    for command in commands {
        if command.is_applicable(&battlestate) {
            command.apply(&mut battlestate);
        } else {
            panic!("Command is not applicable!")
        }
    }
}
