use std::time::Duration;

use gamedata::creatures::sounds::CreatureSound;

use crate::grid::GridPos;

use super::animation::Animation;
use super::movement::Movement;
use super::time_progress::TimeProgress;

pub enum AnimationEvent {
    Instant(InstantEvent),
    TimeProgress(TimeProgressEvent),
    Delay(Duration),
}

pub enum TimeProgressEvent {
    Animation(Animation),
    Movement(Movement),
}

pub enum InstantEvent {
    InvertSide,
    PlaySound(Sound),
    StopSound,
    Teleport(GridPos),
}

pub struct Sound {
    pub type_: CreatureSound,
    pub looping: bool,
}

impl AnimationEvent {
    pub fn animation(animation: Animation) -> Self {
        Self::TimeProgress(TimeProgressEvent::Animation(animation))
    }

    pub fn movement(movement: Movement) -> Self {
        Self::TimeProgress(TimeProgressEvent::Movement(movement))
    }

    pub fn delay(duration: Duration) -> Self {
        Self::Delay(duration)
    }

    pub fn invert_side() -> Self {
        Self::Instant(InstantEvent::InvertSide)
    }

    pub fn play_sound(sound_type: CreatureSound) -> Self {
        Self::play_sound_internal(sound_type, false)
    }

    pub fn play_sound_looping(sound_type: CreatureSound) -> Self {
        Self::play_sound_internal(sound_type, true)
    }

    fn play_sound_internal(sound_type: CreatureSound, looping: bool) -> Self {
        Self::Instant(InstantEvent::PlaySound(Sound {
            type_: sound_type,
            looping,
        }))
    }

    pub fn stop_sound() -> Self {
        Self::Instant(InstantEvent::StopSound)
    }

    pub fn teleport(position: GridPos) -> Self {
        Self::Instant(InstantEvent::Teleport(position))
    }
}

impl TimeProgressEvent {
    pub fn progress(&self) -> &TimeProgress {
        match self {
            Self::Animation(animation) => animation.progress(),
            Self::Movement(movement) => movement.progress(),
        }
    }

    pub fn progress_mut(&mut self) -> &mut TimeProgress {
        match self {
            Self::Animation(animation) => animation.progress_mut(),
            Self::Movement(movement) => movement.progress_mut(),
        }
    }
}
