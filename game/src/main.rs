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
use graphics::Graphics;
use registry::ResourceRegistry;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl::Context::init()?;

    // Инициализация видео подсистемы
    let mut canvas = sdl_context.canvas()?;
    let texture_creator = canvas.texture_creator();

    // Инициализация системы рендера шрифтов
    let font = sdl_context.load_font("/usr/share/fonts/TTF/OpenSans-Bold.ttf", 16)?;

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
