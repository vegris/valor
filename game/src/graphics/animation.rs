use std::time::Duration;

use super::spritesheet::creature::{AnimationType, Creature};

// Idle animation
// 10 frames per second
// 10 frames per 1000 ms
// 1 frame per 100 ms

#[derive(Clone, Copy, Debug)]
pub struct Anim {
    pub type_: AnimationType,
    duration: Duration,
    spent: Duration,
}

pub enum Status {
    Progress(f32),
    Finished,
}

impl Anim {
    pub fn new(animation_type: AnimationType, spritesheet: &Creature) -> Self {
        let frames = spritesheet.frames_count(animation_type).unwrap();
        let duration = Duration::from_millis(100) * frames as u32;

        dbg!(frames, duration);

        Self {
            type_: animation_type,
            duration,
            spent: Duration::ZERO,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.spent += dt;
    }

    pub fn status(&self) -> Status {
        if self.spent >= self.duration {
            Status::Finished
        } else {
            let progress = self.spent.as_secs_f32() / self.duration.as_secs_f32();
            Status::Progress(progress)
        }
    }
}

impl Status {
    pub fn progress(&self) -> f32 {
        match self {
            Self::Progress(progress) => *progress,
            Self::Finished => 1.0
        }
    }
}
