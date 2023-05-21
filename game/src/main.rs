use std::{error::Error, time::Instant};

mod animations;
mod animator;
mod battlestate;
mod command;
mod config;
mod creature_stack;
mod graphics;
mod pathfinding;
mod registry;

extern crate sdl2;

use battlestate::{BattleState, Graphics};
use config::Config;
use registry::ResourceRegistry;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;

    // Инициализация видео подсистемы
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust", 800, 600)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let texture_creator = canvas.texture_creator();

    // Инициализация системы рендера шрифтов
    let ttf_context = sdl2::ttf::init()?;
    let font = ttf_context.load_font("/usr/share/fonts/TTF/OpenSans-Bold.ttf", 16)?;

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    let config = Config::load()?;

    let mut current_state = BattleState::new(&config)?;
    let graphics = Graphics::init(&config, &mut resource_registry, &texture_creator)?;

    let mut last_time = Instant::now();
    loop {
        let current_time = Instant::now();
        // Обработка ввода
        let frame_input = current_state.gather_input(&mut event_pump);
        let frame_data = current_state.process_input(frame_input);

        // Обновление игрового состояния
        current_state.update(current_time - last_time, &mut resource_registry);

        // Отображение игрового состояния
        canvas.clear();
        current_state.draw(
            frame_data,
            &graphics,
            &mut canvas,
            &mut resource_registry,
            &texture_creator,
            &font,
        )?;
        canvas.present();

        last_time = current_time;
    }
}
