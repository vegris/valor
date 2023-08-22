use std::error::Error;
use std::time::Instant;

mod battlestate;
mod command;
mod config;
mod event;
mod graphics;
mod grid;
mod input;
mod map;
mod pathfinding;
mod registry;
mod sound;
mod stack;

use battlestate::BattleState;
use config::Config;
use graphics::{Animations, Statics};
use registry::ResourceRegistry;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    sound::initialize()?;

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

    let statics = Statics::init(
        &config,
        &mut resource_registry,
        &texture_creator,
        &ttf_context,
    )?;

    let mut game_state = BattleState::new(&config)?;

    let mut animations = Animations::create(&game_state, &mut resource_registry);

    if config.music {
        sound::setup_music(&mut resource_registry)?;
    }

    let mut frame_start = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - frame_start;
        frame_start = now;

        let frame_data = input::process(&game_state, &mut event_pump);

        animations.update(dt, &mut resource_registry);

        canvas.clear();
        graphics::draw(
            &game_state,
            &frame_data,
            &animations,
            &mut canvas,
            &mut resource_registry,
            &texture_creator,
            &statics,
        )?;

        canvas.present();

        if !animations.is_animating() {
            if let Some(command) = frame_data.command {
                let events = game_state.apply_command(command);

                graphics::process_events(
                    &game_state,
                    events,
                    &mut animations,
                    &mut resource_registry,
                );
            }
        }
    }
}
