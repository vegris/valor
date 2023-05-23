use std::error::Error;

mod battlestate;
mod command;
mod config;
mod creature_stack;
mod graphics;
mod grid;
mod input;
mod pathfinding;
mod registry;

extern crate sdl2;

use battlestate::BattleState;
use config::Config;
use graphics::Graphics;
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

    loop {
        // Обработка ввода
        let frame_input = input::gather_input(&mut event_pump);
        let frame_data = current_state.process_input(frame_input);

        // Отображение игрового состояния
        canvas.clear();
        graphics.draw(
            &current_state,
            frame_data,
            &mut canvas,
            &mut resource_registry,
            &texture_creator,
            &font,
        )?;
        canvas.present();
    }
}
