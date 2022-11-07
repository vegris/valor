use std::{error::Error, time::Instant};

mod creature_stack;
mod battlestate;
mod command;
mod pathfinding;
mod registry;
mod graphics;
mod config;
mod animations;
mod animator;

extern crate sdl2;

use battlestate::BattleState;
use registry::ResourceRegistry;
use config::Config;

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

    let config = Config::load_config()?;

    let mut current_state = BattleState::new(config, &mut resource_registry, &texture_creator)?;

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
        current_state.draw(frame_data, &mut canvas, &mut resource_registry, &texture_creator, &font)?;
        canvas.present();

        last_time = current_time;
    }
}
