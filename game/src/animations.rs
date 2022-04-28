use std::time::{Instant, Duration};

use gridpos::GridPos;
use sdl2::rect::Point;

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


impl Animation {
    const DURATION: Duration = Duration::from_millis(650);

    fn calculate_progress(&self, now: Instant) -> f32 {
        (self.start + Self::DURATION - now).as_secs_f32() / Self::DURATION.as_secs_f32()
    }

    pub fn state(&self, now: Instant) -> AnimationState {
        if self.start > now {
            AnimationState::NotStarted
        } else if (self.start + Self::DURATION) < now {
            AnimationState::Finished
        } else {
            AnimationState::Running(self.calculate_progress(now))
        }
    }
}


#[derive(Clone, Debug)]
pub struct Tweening {
    pub path: Vec<GridPos>,
    pub start: Instant
}

pub enum TweeningState {
    NotStarted,
    Running(Point),
    Finished
}

impl Tweening {
    const DURATION: Duration = Duration::from_secs(1);

    fn calculate_progress(&self, now: Instant) -> Point {
        let total_duration = Self::DURATION * self.path.len() as u32;

        let relative_time = self.start + total_duration - now;
        let completion_percentage = 1.0 - relative_time.as_secs_f32() / total_duration.as_secs_f32();
        
        let current_index = (completion_percentage * self.path.len() as f32).floor() as usize;

        self.path[current_index].center()
    }

    pub fn state(&self, now: Instant) -> TweeningState {
        if self.start > now {
            TweeningState::NotStarted
        } else if (self.start + Self::DURATION * self.path.len() as u32) < now {
            TweeningState::Finished
        } else {
            TweeningState::Running(self.calculate_progress(now))
        }
    }
}