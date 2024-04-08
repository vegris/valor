use std::time::Instant;

use common::error::AnyWay;

mod config;
mod graphics;
mod gridpos;
mod gui;
mod input;
mod registry;
mod sound;

use config::Config;
use graphics::{animations::entity_animations::EntityAnimations, Animations, Statics};
use logic::gamestate::GameState;
use registry::ResourceRegistry;

pub enum State {
    Main,
    SpellBook,
}

fn main() -> AnyWay {
    let config = Config::load()?;

    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    sound::initialize(&config)?;

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

    let mut game_state = GameState::new(&config.armies)?;

    let mut animations = Animations::create(&game_state, &mut resource_registry);

    let mut entity_animations = EntityAnimations::new();

    if config.music {
        sound::setup_music(&mut resource_registry)?;
    }

    let ctx = gui::create_context();

    let mut frame_start = Instant::now();

    let mut state = State::Main;

    loop {
        let now = Instant::now();
        let dt = now - frame_start;
        frame_start = now;

        let frame_input = input::gather_input(&mut event_pump);

        let mut cast = None;
        let full_output = gui::create_frame(&ctx, &frame_input, &mut state, &mut cast);

        let frame_data = input::process_input(&game_state, &frame_input, &mut state, cast);

        animations.update(dt, &mut resource_registry);
        entity_animations.update(dt, &mut resource_registry);

        let shapes = gui::output_to_shapes(full_output);

        canvas.clear();
        graphics::draw(
            &game_state,
            &frame_data,
            &animations,
            &entity_animations,
            &mut canvas,
            &mut resource_registry,
            &texture_creator,
            &statics,
            shapes,
            &state,
        )?;

        canvas.present();

        if !animations.is_animating() {
            if let Some(command) = frame_data.command {
                let events = game_state.apply_command(command);

                graphics::process_events(
                    &game_state,
                    events,
                    &mut animations,
                    &mut entity_animations,
                    &mut resource_registry,
                );
            }
        }
    }
}
