use std::error::Error;
use std::path::PathBuf;

use rand::seq::IteratorRandom;
use sdl2::mixer;
use sdl2::mixer::Channel;

use crate::registry::ResourceRegistry;

static mut MUSIC_TRACK: Option<mixer::Music> = None;

const START_CHANNEL: Channel = Channel(1);
const LOOPING_CHANNEL: Channel = Channel(2);

pub fn initialize() -> Result<(), Box<dyn Error>> {
    mixer::init(mixer::InitFlag::MP3)?;

    mixer::open_audio(
        mixer::DEFAULT_FREQUENCY,
        mixer::DEFAULT_FORMAT,
        mixer::DEFAULT_CHANNELS,
        256,
    )?;

    mixer::allocate_channels(8);
    mixer::reserve_channels(2);

    Ok(())
}

pub fn setup_music(rr: &mut ResourceRegistry) -> Result<(), Box<dyn Error>> {
    START_CHANNEL.play(rr.get_sound(&start_sound()), 0)?;

    let music = mixer::Music::from_file(music_track()).ok();

    unsafe {
        MUSIC_TRACK = music;
    }

    mixer::set_channel_finished(|ch| {
        if ch.0 == 1 {
            let music = unsafe { MUSIC_TRACK.as_ref().unwrap() };
            music.play(-1).unwrap();
            sdl2::mixer::unset_channel_finished();
        }
    });

    Ok(())
}

pub fn play_sound(
    filename: &str,
    rr: &mut ResourceRegistry,
    looping: bool,
) -> Result<(), Box<dyn Error>> {
    let (channel, loops) = if looping {
        (LOOPING_CHANNEL, -1)
    } else {
        (sdl2::mixer::Channel(-1), 0)
    };

    channel.play(rr.get_sound(filename), loops)?;

    Ok(())
}

pub fn stop_looping() {
    LOOPING_CHANNEL.halt();
}

fn start_sound() -> String {
    const BATTLE_START_TRACK_COUNT: usize = 8;

    (0..BATTLE_START_TRACK_COUNT)
        .choose(&mut rand::thread_rng())
        .map(|choice| format!("BATTLE0{}", choice))
        .unwrap()
}

fn music_track() -> PathBuf {
    const MUSIC_PATH: &str = "/home/vsevolod/Games/HoMM3/drive_c/Games/HoMM 3 Complete/Mp3";
    const MUSIC_TRACK_COUNT: usize = 4;

    let track_name = (1..=MUSIC_TRACK_COUNT)
        .choose(&mut rand::thread_rng())
        .map(|choice| format!("COMBAT0{}.MP3", choice))
        .unwrap();

    [MUSIC_PATH, &track_name].iter().collect()
}
