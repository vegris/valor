#![allow(unused)]
use std::time::Instant;
use std::error::Error;

mod creature_stack;
mod battlestate;
mod command;
mod functions;
mod gridpos;
mod resources;
mod battlefields;
mod graphics;

extern crate sdl2;

use battlestate::BattleState;
use creature::Creature;
use gridpos::GridPos;
use resources::ResourceRegistry;
use battlefields::Battlefield;

fn main() -> Result<(), Box<dyn Error>> {
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
    let attacker_units = [
            Some((Creature::Angel, 8)),
            Some((Creature::Angel, 10)),
            Some((Creature::RoyalGriffin, 30)),
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
            Some((Creature::Hydra, 25)),
            Some((Creature::HornedDemon, 25)),
            None
        ];

    let mut current_state = BattleState::new(
        attacker_units, defender_units,
        &mut resource_registry, &texture_creator, Battlefield::GRMT
    )?;

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
