use std::{mem::MaybeUninit, ptr::addr_of_mut, time::Instant};

use common::error::{AnyHow, AnyWay};

mod config;
mod graphics;
mod gridpos;
mod gui;
mod input;
mod resources;
mod sound;

use config::Config;
use graphics::{animations::entity_animations::EntityAnimations, Animations, Statics};
use logic::gamestate::GameState;
use resources::ResourceRegistry;
use sdl2::{
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

pub enum Stage {
    Main,
    SpellBook,
}

pub struct Graphics<'a> {
    statics: Statics<'a>,
    texture_creator: TextureCreator<WindowContext>,
    canvas: WindowCanvas,
}

impl<'a> Graphics<'a> {
    fn init(
        sdl_context: &sdl2::Sdl,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        config: &Config,
        resource_registry: &mut ResourceRegistry,
    ) -> AnyHow<Self> {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        let canvas = sdl_context
            .video()?
            .window("Rust", 800, 600)
            .position_centered()
            .build()?
            .into_canvas()
            .present_vsync()
            .build()?;

        let canvas = unsafe {
            addr_of_mut!((*ptr).canvas).write(canvas);
            &(*ptr).canvas
        };

        let texture_creator = canvas.texture_creator();

        let texture_creator = unsafe {
            addr_of_mut!((*ptr).texture_creator).write(texture_creator);
            &(*ptr).texture_creator
        };

        // Statics keep reference to TextureCreator, so TextureCreator must already be in it's
        // place when Statics are initialized
        let statics = Statics::init(config, resource_registry, texture_creator, ttf_context)?;
        unsafe { addr_of_mut!((*ptr).statics).write(statics) };

        unsafe { Ok(uninit.assume_init()) }
    }
}

fn main() -> AnyWay {
    let config = Config::load()?;

    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    sound::initialize(&config)?;

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    let mut graphics_ =
        Graphics::init(&sdl_context, &ttf_context, &config, &mut resource_registry)?;

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    let mut game_state = GameState::new(&config.armies)?;

    let mut animations = Animations::create(&game_state, &mut resource_registry);

    let mut entity_animations = EntityAnimations::new();

    if config.music {
        sound::setup_music(&mut resource_registry)?;
    }

    let ctx = egui::Context::default();

    let mut frame_start = Instant::now();

    let mut stage = Stage::Main;

    loop {
        let now = Instant::now();
        let dt = now - frame_start;
        frame_start = now;

        let frame_input = input::gather_input(&mut event_pump);

        let mut cast = None;
        let full_output = gui::create_frame(&ctx, &frame_input, &mut stage, &mut cast);

        let frame_data = input::process_input(&game_state, &frame_input, &mut stage, cast);

        animations.update(dt, &mut resource_registry);
        entity_animations.update(dt, &mut resource_registry);

        let shapes = gui::output_to_shapes(full_output);

        graphics_.canvas.clear();
        graphics::draw(
            &game_state,
            &frame_data,
            &mut graphics_,
            &animations,
            &entity_animations,
            &mut resource_registry,
            shapes,
            &stage,
        )?;

        graphics_.canvas.present();

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
