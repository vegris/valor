use std::{time::Duration, collections::VecDeque};

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

    pub fn is_finished(&self) -> bool {
        matches!(self.state(), AnimationState::Finished)
    }

    pub fn is_blocking(&self) -> bool {
        ![AnimationType::Standing, AnimationType::MouseOver].contains(&self.type_)
    }
}

#[derive(Clone, Debug)]
pub struct AnimationQueue(VecDeque<Animation>);

impl AnimationQueue {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn update(&mut self, dt: Duration) {
        if let Some(animation) = self.0.front_mut() {
            animation.update(dt);
        }
    }

    pub fn remove_finished(&mut self) {
        if let Some(animation) = self.0.front_mut() {
            if animation.is_finished() {
                self.0.pop_front();
            }
        }
    }

    pub fn remove_non_blocking(&mut self) {
        if let Some(animation) = self.current() {
            if !animation.is_blocking() {
                self.0.pop_front();
            }
        }
    }

    pub fn add_standing(&mut self) {
        if self.0.is_empty() {
            self.0.push_back(Animation::new(AnimationType::Standing));
        }
    }

    pub fn add(&mut self, animation: Animation) {
        self.remove_non_blocking();
        self.0.push_back(animation);
    }

    pub fn current(&self) -> Option<Animation> {
        self.0.front().copied()
    }
}
