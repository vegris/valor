#[allow(unused)]
use std::error::Error;

mod creature;
mod creature_stack;
mod battlestate;
mod command;
mod gridpos;
mod pathfinding;
mod registry;
mod battlefields;
mod graphics;

extern crate sdl2;

use battlestate::BattleState;
use creature::Creature;
use registry::ResourceRegistry;
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

    // Инициализация системы рендера шрифтов
    let ttf_context = sdl2::ttf::init()?;
    let font = ttf_context.load_font("/usr/share/fonts/liberation/LiberationMono-Bold.ttf", 16)?;

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    // Создание начального игрового состояния
    let attacker_units = [
            Some((Creature::Archer, 55)),
            None, // Some((Creature::Angel, 8)),
            None, // Some((Creature::Angel, 10)),
            Some((Creature::RoyalGriffin, 30)),
            None,
            None,
            None
        ];

    let defender_units = [
            None,
            None,
            Some((Creature::Devil, 10)),
            Some((Creature::Angel, 20)),
            Some((Creature::Hydra, 1)),
            Some((Creature::HornedDemon, 25)),
            None
        ];

    let mut current_state = BattleState::new(
        attacker_units, defender_units,
        &mut resource_registry, &texture_creator, Battlefield::GRMT
    )?;

    // let mut last_time = Instant::now();
    loop {
        // let current_time = Instant::now();
        // Обработка ввода
        let frame_input = current_state.gather_input(&mut event_pump);
        let frame_data = current_state.process_input(frame_input);

        // Обновление игрового состояния
        // current_state.update(current_time - last_time, &mut resource_registry);
        
        // Отображение игрового состояния
        canvas.clear();
        current_state.draw(frame_data, &mut canvas, &mut resource_registry, &texture_creator, &font)?;
        canvas.present();

        // last_time = current_time;
    }
}
