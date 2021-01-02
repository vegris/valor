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


fn main_server() {
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
use std::time::Instant;

extern crate sdl2;

mod resources;
use resources::ResourceRegistry;

mod enumerations;
use enumerations::Battlefield;

mod gamestate;
use gamestate::BattleState as BattleStateGraphics;

mod util;
use util::AnyError;

mod graphics;


fn main_graphics() -> Result<(), AnyError> {
    let sdl_context = sdl2::init()?; 

    // Инициализация видео подсистемы
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust", 800, 600)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()?;
    let texture_creator = canvas.texture_creator();

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    // Создание начального игрового состояния
    let mut current_state = BattleStateGraphics::new(&mut resource_registry, &texture_creator, Battlefield::GRMT)?;

    let mut last_time = Instant::now();
    loop {
        let current_time = Instant::now();
        // Обработка ввода
        current_state.process_input(&mut event_pump);

        // Обновление игрового состояния
        current_state.update(current_time - last_time, &mut resource_registry);
        
        // Отображение игрового состояния
        canvas.clear();
        current_state.draw(&mut canvas, &mut resource_registry, &texture_creator)?;
        canvas.present();

        last_time = current_time;
    }
}

fn main() -> Result<(), AnyError> {
    main_server();
    main_graphics()?;
    Ok(())
}