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
mod stack;

use rand::seq::IteratorRandom;

use battlestate::BattleState;
use config::Config;
use graphics::Statics;
use registry::ResourceRegistry;

const MUSIC_PATH: &str = "/home/vsevolod/Games/HoMM3/drive_c/Games/HoMM 3 Complete/Mp3";
const MUSIC_TRACK_COUNT: usize = 4;
const BATTLE_START_TRACK_COUNT: usize = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    // Инициализация SDL
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init()?;

    let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3)?;
    sdl2::mixer::open_audio(
        sdl2::mixer::DEFAULT_FREQUENCY,
        sdl2::mixer::DEFAULT_FORMAT,
        sdl2::mixer::DEFAULT_CHANNELS,
        256,
    )?;

    sdl2::mixer::allocate_channels(4);
    sdl2::mixer::reserve_channels(1);

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

    let channel = 1;
    let sound = resource_registry.get_sound(&choose_battle_start_sound());
    sdl2::mixer::Channel(channel).play(sound, 0)?;

    let music = std::rc::Rc::new(sdl2::mixer::Music::from_file(choose_music())?);
    let music_copy = music.clone();
    sdl2::mixer::set_channel_finished(move |ch| {
        if ch.0 == 1 {
            music_copy.play(-1).unwrap();
            sdl2::mixer::unset_channel_finished();
        }
    });

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

    let mut frame_start = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - frame_start;
        frame_start = now;

        let frame_data = input::process(&game_state, &mut event_pump);

        // game_state.update(dt, &mut resource_registry);

        for animation_state in animations.values_mut() {
            animation_state.update(dt, &mut resource_registry);
        }

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

        if let Some(command) = frame_data.command {
            let events = game_state.apply_command(command);

            graphics::process_events(&game_state, events, &mut animations, &mut resource_registry);
        }
    }
}

fn choose_battle_start_sound() -> String {
    let mut rng = rand::thread_rng();
    let choice = (0..BATTLE_START_TRACK_COUNT).choose(&mut rng).unwrap();
    format!("BATTLE0{}", choice)
}

fn choose_music() -> String {
    let mut rng = rand::thread_rng();

    let choice = (1..=MUSIC_TRACK_COUNT).choose(&mut rng).unwrap();

    let track_name = format!("COMBAT0{}.MP3", choice);

    let mut path = std::path::PathBuf::new();
    path.push(MUSIC_PATH);
    path.push(track_name);

    path.into_os_string().into_string().unwrap()
}
