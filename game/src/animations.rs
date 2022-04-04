use std::time::{Instant, Duration};

use crate::graphics::creature::AnimationType;

#[derive(Clone, Copy, Debug)]
pub struct Animation {
    pub type_: AnimationType,
    pub start: Instant
}

pub enum AnimationState {
    NotStarted,
    Running(f32),
    Finished
}

const ANIMATION_DURATION: Duration = Duration::from_millis(650);

impl Animation {
    fn calculate_progress(&self, now: Instant) -> f32 {
        (self.start + ANIMATION_DURATION - now).as_secs_f32() / ANIMATION_DURATION.as_secs_f32()
    }

    pub fn state(&self, now: Instant) -> AnimationState {
        if self.start > now {
            AnimationState::NotStarted
        } else if (self.start + ANIMATION_DURATION) < now {
            AnimationState::Finished
        } else {
            AnimationState::Running(self.calculate_progress(now))
        }
    }
}