use std::mem::ManuallyDrop;
use std::path::PathBuf;

use rand::seq::IteratorRandom;
use sdl2::mixer;
use sdl2::mixer::Channel;

use crate::config::Config;
use crate::error::AnyWay;
use crate::registry::ResourceRegistry;

const START_CHANNEL: Channel = Channel(0);
const LOOPING_CHANNEL: Channel = Channel(1);
const TOTAL_CHANNELS: i32 = 8;

pub fn initialize(config: &Config) -> AnyWay {
    mixer::init(mixer::InitFlag::MP3)?;

    mixer::open_audio(
        mixer::DEFAULT_FREQUENCY,
        mixer::DEFAULT_FORMAT,
        mixer::DEFAULT_CHANNELS,
        256,
    )?;

    mixer::Music::set_volume(config.volume);

    mixer::allocate_channels(TOTAL_CHANNELS);
    mixer::reserve_channels(2);

    for channel in 0..TOTAL_CHANNELS {
        Channel(channel).set_volume(config.volume);
    }

    Ok(())
}

pub fn setup_music(rr: &mut ResourceRegistry) -> AnyWay {
    let sound = ManuallyDrop::new(rr.load_sound(&start_sound())?);
    START_CHANNEL.play(&sound, 0)?;

    let music = ManuallyDrop::new(mixer::Music::from_file(music_track())?);
    mixer::set_channel_finished(move |ch| {
        if ch == START_CHANNEL {
            music.play(-1).unwrap();
            sdl2::mixer::unset_channel_finished();
        }
    });

    Ok(())
}

pub fn play_sound(chunk: &mixer::Chunk, looping: bool) -> AnyWay {
    let (channel, loops) = if looping {
        (LOOPING_CHANNEL, -1)
    } else {
        (Channel(-1), 0)
    };

    channel.play(chunk, loops)?;

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
    const MUSIC_PATH: &str = "/home/vsevolod/Games/HoMM3/drive_c/Games/HoMM3/Mp3";
    const MUSIC_TRACK_COUNT: usize = 4;

    let track_name = (1..=MUSIC_TRACK_COUNT)
        .choose(&mut rand::thread_rng())
        .map(|choice| format!("COMBAT0{}.MP3", choice))
        .unwrap();

    [MUSIC_PATH, &track_name].iter().collect()
}
