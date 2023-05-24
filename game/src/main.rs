use std::error::Error;

mod battlestate;
mod command;
mod config;
mod graphics;
mod grid;
mod input;
mod pathfinding;
mod registry;
mod sdl;
mod stack;

extern crate sdl2;

use battlestate::BattleState;
use config::Config;
use graphics::Statics;
use registry::ResourceRegistry;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    let sdl_context = sdl::Context::init()?;

    // Инициализация видео подсистемы
    let mut canvas = sdl_context.canvas()?;
    let texture_creator = canvas.texture_creator();

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    let mut current_state = BattleState::new(&config)?;

    let statics = Statics::init(
        &config,
        &sdl_context,
        &mut resource_registry,
        &texture_creator,
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
