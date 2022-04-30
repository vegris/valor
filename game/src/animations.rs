use std::time::Duration;

use gridpos::GridPos;

use crate::graphics::creature::AnimationType;

#[derive(Clone, Copy, Debug)]
pub struct Tweening {
    pub from: GridPos,
    pub to: GridPos
}

#[derive(Clone, Copy, Debug)]
pub struct Animation {
    pub type_: AnimationType,
    pub duration: Duration,
    pub tween: Option<Tweening>
}

pub enum AnimationState {
    Running(f32),
    Finished
}


impl Animation {
    const DURATION: Duration = Duration::from_millis(500);
    // const DURATION: Duration = Duration::from_secs(5);

    pub fn new(type_: AnimationType) -> Self {
        Self { type_, duration: Duration::ZERO, tween: None }
    }

    pub fn new_with_tween(type_: AnimationType, from: GridPos, to: GridPos) -> Self {
        Self { type_, duration: Duration::ZERO, tween: Some(Tweening { from, to }) }
    }

    pub fn update(&mut self, dt: Duration) {
        self.duration += dt;
    }

    fn calculate_progress(&self) -> f32 {
        self.duration.as_secs_f32() / Self::DURATION.as_secs_f32()
    }

    pub fn state(&self) -> AnimationState {
        if self.duration <= Self::DURATION {
            AnimationState::Running(self.calculate_progress())
        } else {
            AnimationState::Finished
        }
    }
}
