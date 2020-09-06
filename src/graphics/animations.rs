use std::time::Duration;

use crate::graphics::creature::AnimationType;


struct TimeProgress {
    delay: Option<Duration>,
    total_duration: Duration,
    time_passed: Duration
}

enum TimeProgressState {
    Delayed,
    Going(f32),
    Finished
}

impl TimeProgress{
    fn new(duration: Duration, delay: Option<Duration>) -> Self {
        Self {
            delay,
            total_duration: duration,
            time_passed: Duration::from_secs(0)
        }
    }

    fn reset(&mut self) {
        self.time_passed = Duration::from_secs(0);
    }

    fn update(&mut self, dt: Duration) {
        if let Some(delay) = self.delay {
            self.delay = delay.checked_sub(dt);
        } else {
            self.time_passed += dt;
        }
    }

    fn progress(&self) -> TimeProgressState {
        if self.delay.is_some() {
            TimeProgressState::Delayed
        } else if self.time_passed < self.total_duration {
            let progress = self.time_passed.as_secs_f32() / self.total_duration.as_secs_f32();
            TimeProgressState::Going(progress)
        } else {
            TimeProgressState::Finished
        }
    }

    fn is_delayed(&self) -> bool {
        matches!(self.progress(), TimeProgressState::Delayed)
    }

    fn is_finished(&self) -> bool {
        matches!(self.progress(), TimeProgressState::Finished)
    }
}

pub struct CreatureAnimation {
    animation_type: AnimationType,
    time_progress: TimeProgress,
    looping: bool
}

impl CreatureAnimation {
    pub fn new(animation_type: AnimationType) -> Self {
        Self::_new(animation_type, false, None)
    }
    pub fn new_looping(animation_type: AnimationType) -> Self {
        Self::_new(animation_type, true, None)
    }
    pub fn new_delayed(animation_type: AnimationType, delay: Duration) -> Self {
        Self::_new(animation_type, false, Some(delay))
    }
    fn _new(animation_type: AnimationType, looping: bool, delay: Option<Duration>) -> Self {
        Self {
            animation_type,
            time_progress: TimeProgress::new(animation_type.duration(), delay),
            looping
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.time_progress.update(dt);
        if self.is_looping() && self.time_progress.is_finished() {
            self.time_progress.reset();
        }
    }

    pub fn animation_type(&self) -> AnimationType {
        self.animation_type
    }

    pub fn progress(&self) -> Option<f32> {
        if let TimeProgressState::Going(progress) = self.time_progress.progress() {
            Some(progress)
        } else {
            None
        }
    }

    pub fn is_delayed(&self) -> bool {
        self.time_progress.is_delayed()
    }

    pub fn is_finished(&self) -> bool {
        self.time_progress.is_finished()
    }

    pub fn is_looping(&self) -> bool {
        self.looping
    }
}