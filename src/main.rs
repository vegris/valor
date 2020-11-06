#![allow(unused)]

#[macro_use]
extern crate num_derive;

mod creature;
mod creature_stack;
mod skills;
mod action_queue;
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

    let mut attacker_army = Army::new(attacker_hero, [
            Some((Creature::Angel, 8)),
            Some((Creature::Angel, 10)),
            None,
            None,
            Some((Creature::Archer, 55)),
            None,
            None
        ],
    );

    let mut defender_army = Army::new(defender_hero, [
            None,
            None,
            Some((Creature::Devil, 10)),
            Some((Creature::Angel, 20)),
            None,
            Some((Creature::HornedDemon, 50)),
            None
        ]
    );

    let mut battlestate = BattleState::new(attacker_army, defender_army);

    let commands = vec![
        // А. Ангелы на 3,7
        Command::new(Side::Attacker, CommandType::Move { destination: (3, 7) }),
        // З. Дьяволы на 7, 9
        Command::new(Side::Defender, CommandType::Move { destination: (7, 9) }),
        // З. Демоны ждут
        Command::new(Side::Defender, CommandType::Wait),
        // А. Арбалетчики стреляют в демонов
        // демоны - в пятом слоте вражеской армии
        Command::new(Side::Attacker, CommandType::Shoot { target: 5 }),
        // 3. Демоны на 5, 7
        Command::new(Side::Defender, CommandType::Move { destination: (5, 7) }),
        
        // Новый ход

        // А. Ангелы атакуют демонов стоя на 4, 7
        Command::new(Side::Attacker, CommandType::Attack { position: (4, 7), target: 5 })
    ];

    for command in commands {
        if battlestate.is_applicable(command) {
            battlestate.apply(command);
        } else {
            panic!("Command is not applicable!")
        }
    }
}

    // // Фазы ходов существ
    // let battle_phases = [CreatureTurnState::HasTurn, CreatureTurnState::MoraledAndWaited, CreatureTurnState::Waited];
    // let phase_iter = battle_phases.iter();//.cycle();

    // // Кто начинал первым в прошлый ход
    // let last_turn_side = Side::Defender;

    // for current_phase in phase_iter {
    //     dbg!(current_phase);
    //     while let Some(stack) = functions::find_current_creature(
    //         &mut attacker_army.battle_army, 
    //         &mut defender_army.battle_army,
    //         last_turn_side,
    //         *current_phase
    //     ) {
    //         stack.turn_phase = match current_phase {
    //             CreatureTurnState::HasTurn => CreatureTurnState::Waited,
    //             _ => CreatureTurnState::NoTurn
    //         };
    //         dbg!((stack.creature(), stack.count(), stack.turn_phase));
    //     }
    // }
// }
