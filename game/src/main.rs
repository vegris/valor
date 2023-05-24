use std::error::Error;

mod battlestate;
mod command;
mod config;
mod graphics;
mod grid;
mod input;
mod pathfinding;
mod registry;
mod stack;

use battlestate::BattleState;
use config::Config;
use graphics::Statics;
use registry::ResourceRegistry;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;

    // Инициализация видео подсистемы
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    let texture_creator = canvas.texture_creator();

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    let mut current_state = BattleState::new(&config)?;

    let statics = Statics::init(
        &config,
        &mut resource_registry,
        &texture_creator,
        &ttf_context,
    )?;

    loop {
        // Обработка ввода
        let frame_input = input::gather_input(&mut event_pump);
        let frame_data = current_state.process_input(frame_input);

        // Отображение игрового состояния
        canvas.clear();
        graphics::draw(
            &current_state,
            frame_data,
            &mut canvas,
            &mut resource_registry,
            &texture_creator,
            &statics,
        )?;
        canvas.present();
    }
}
