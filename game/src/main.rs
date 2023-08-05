use std::error::Error;
use std::time::Instant;

mod battlestate;
mod command;
mod config;
mod event;
mod graphics;
mod grid;
mod input;
mod pathfinding;
mod registry;
mod sound;
mod stack;

use battlestate::BattleState;
use config::Config;
use graphics::{spritesheet::button_state::ButtonState, statics::Buttons, Statics};
use registry::ResourceRegistry;
use sdl2::rect::Rect;

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

    let mut game_state = BattleState::new(&config)?;

    let statics = Statics::init(
        &config,
        &mut resource_registry,
        &texture_creator,
        &ttf_context,
    )?;

    let mut animations = graphics::create_animations(&game_state, &mut resource_registry);

    sound::setup_music(&mut resource_registry)?;

    // let menu_background = resource_registry.load_pcx("coplacbr.pcx")?;
    let menu_background = resource_registry.load_pcx("cbar.pcx")?;

    let mut frame_start = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - frame_start;
        frame_start = now;

        let frame_data = input::process(&game_state, &mut event_pump);

        for animation_state in animations.values_mut() {
            animation_state.update(dt, &mut resource_registry);
        }

        let is_animating = animations.0.values().any(|a| a.is_animating());

        canvas.clear();
        graphics::draw(
            &game_state,
            &frame_data,
            &animations,
            &mut canvas,
            &mut resource_registry,
            &texture_creator,
            &statics,
            is_animating,
        )?;

        let texture = menu_background.as_texture(&texture_creator)?;
        canvas.copy(
            &texture,
            None,
            // Rect::new(0, 556, menu_background.width(), menu_background.height()),
            Rect::new(1, 555, menu_background.width(), menu_background.height()),
        )?;

        let buttons = [
            (Buttons::Settings, 4),
            (Buttons::Surrender, 55),
            (Buttons::Retreat, 106),
            (Buttons::AutoBattle, 157),
            (Buttons::BookOfMagic, 646),
            (Buttons::Wait, 697),
            (Buttons::Defend, 748),
        ];

        for (button, x) in buttons {
            let sprite = statics.ui[button].get_sprite(ButtonState::Base, 0).unwrap();
            let texture = sprite.surface.as_texture(&texture_creator)?;

            canvas.copy(
                &texture,
                None,
                Rect::new(x, 560, sprite.width, sprite.height),
            )?;
        }

        canvas.present();

        if !is_animating {
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
