use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::addr_of_mut;
use std::time::{Duration, Instant};

use animations::Animations;
use common::error::{AnyHow, AnyWay};

mod animations;
mod config;
mod graphics;
mod gridpos;
mod gui;
mod input;
mod resources;
mod sound;

use config::Config;
use graphics::Statics;
use logic::gamestate::GameState;
use resources::ResourceRegistry;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub enum Stage {
    Main,
    SpellBook,
}

pub struct Graphics<'a> {
    statics: Statics<'a>,
    texture_creator: TextureCreator<WindowContext>,
    canvas: WindowCanvas,
}

struct FrameTimer(Instant);

impl<'a> Graphics<'a> {
    fn init(
        sdl_context: &sdl2::Sdl,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        config: &Config,
        resource_registry: &mut ResourceRegistry,
    ) -> AnyHow<Pin<Box<Self>>> {
        // Safety: fields are defined in the order in which they must be dropped
        // Resulting struct is protected by pinned box
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        let canvas = sdl_context
            .video()?
            .window("Valor", 800, 600)
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

        let graphics = unsafe { uninit.assume_init() };

        Ok(Box::pin(graphics))
    }
}

impl FrameTimer {
    fn init() -> Self {
        Self(Instant::now())
    }

    fn dt(&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.0;
        self.0 = now;

        dt
    }
}

fn main() -> AnyWay {
    let config = Config::load()?;

    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;
    sound::initialize(&config)?;

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init(&config);

    let mut graphics_ =
        Graphics::init(&sdl_context, &ttf_context, &config, &mut resource_registry)?;

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    let mut game_state = GameState::new(&config.armies)?;

    let mut animations = Animations::init(&game_state, &mut resource_registry);

    if config.music {
        sound::setup_music(&config, &mut resource_registry)?;
    }

    let ctx = egui::Context::default();

    let mut stage = Stage::Main;

    let mut frame_timer = FrameTimer::init();

    loop {
        let dt = frame_timer.dt();

        let frame_input = input::gather_input(&mut event_pump);

        let mut cast = None;
        let shapes = gui::create_frame(&ctx, &frame_input, &mut stage, &mut cast);

        let frame_data = input::process_input(&game_state, &frame_input, &mut stage, cast);

        if !animations.is_animating() {
            if let Some(command) = frame_data.command {
                let events = game_state.apply_command(command);

                animations::process_events(
                    &game_state,
                    events,
                    &mut animations,
                    &mut resource_registry,
                );
            }
        }

        animations.update(dt, &mut resource_registry);

        graphics_.canvas.clear();
        graphics::draw(
            &game_state,
            &frame_data,
            &mut graphics_,
            &animations,
            &mut resource_registry,
            shapes,
            &stage,
        )?;
        graphics_.canvas.present();
    }
}
