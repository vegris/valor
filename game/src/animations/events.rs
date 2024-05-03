use std::time::Duration;

use gamedata::creatures;
use logic::grid::GridPos;

use super::animation::Animation;
use super::movement::Movement;
use super::time_progress::TimeProgress;

pub enum AnimationEvent {
    Animation(Animation),
    Movement(Movement),
    Delay(Duration),
    InvertSide,
    PlaySound(Sound),
    StopSound,
    Teleport(GridPos),
}

pub enum AnimationEventByGroup {
    Instant(InstantEvent),
    Time(TimeEvent),
}

pub enum TimeEvent {
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
    pub type_: creatures::Sound,
    pub looping: bool,
}

impl From<AnimationEvent> for AnimationEventByGroup {
    fn from(value: AnimationEvent) -> Self {
        match value {
            AnimationEvent::Animation(animation) => AnimationEventByGroup::Time(
                TimeEvent::TimeProgress(TimeProgressEvent::Animation(animation)),
            ),
            AnimationEvent::Movement(movement) => AnimationEventByGroup::Time(
                TimeEvent::TimeProgress(TimeProgressEvent::Movement(movement)),
            ),
            AnimationEvent::Delay(duration) => {
                AnimationEventByGroup::Time(TimeEvent::Delay(duration))
            }
            AnimationEvent::InvertSide => AnimationEventByGroup::Instant(InstantEvent::InvertSide),
            AnimationEvent::PlaySound(sound) => {
                AnimationEventByGroup::Instant(InstantEvent::PlaySound(sound))
            }
            AnimationEvent::StopSound => AnimationEventByGroup::Instant(InstantEvent::StopSound),
            AnimationEvent::Teleport(position) => {
                AnimationEventByGroup::Instant(InstantEvent::Teleport(position))
            }
        }
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

impl Sound {
    pub fn new(type_: creatures::Sound) -> Self {
        Self::_new(type_, false)
    }

    pub fn new_looping(type_: creatures::Sound) -> Self {
        Self::_new(type_, true)
    }

    fn _new(type_: creatures::Sound, looping: bool) -> Self {
        Self { type_, looping }
    }
}
