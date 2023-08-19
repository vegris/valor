use std::error::Error;
use std::time::{Duration, Instant};

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
mod sdl;
mod sound;
mod stack;

use battlestate::BattleState;
use config::Config;
use graphics::{Animations, Statics};
use registry::ResourceRegistry;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    let sdl = sdl::Sdl::initialize()?;

    let mut canvas = sdl.build_canvas()?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl.main_context.event_pump()?;

    let mut resource_registry = ResourceRegistry::init();

    let statics = Statics::init(
        &config,
        &mut resource_registry,
        &texture_creator,
        &sdl.ttf_context,
    )?;

    let mut game_state = BattleState::new(&config)?;

    let mut animations = Animations::create(&game_state, &mut resource_registry);

    if config.music {
        sound::setup_music(&mut resource_registry)?;
    }

    let mut frame_timer = FrameTimer::new();

    loop {
        let dt = frame_timer.next_frame();

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

struct FrameTimer {
    frame_start: Instant,
}

impl FrameTimer {
    fn new() -> Self {
        Self {
            frame_start: Instant::now(),
        }
    }

    fn next_frame(&mut self) -> Duration {
        let previous_frame = self.frame_start;
        self.frame_start = Instant::now();

        self.frame_start - previous_frame
    }
}
