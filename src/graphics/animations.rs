use std::time::Duration;

use sdl2::rect::Point;

use crate::graphics::creature::AnimationType;
use crate::gamestate::creature::CreatureStack;


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

    fn total_duration(&self) -> Duration {
        let duration_left = self.total_duration - self.time_passed;
        self.delay.map_or(duration_left, |delay| delay + duration_left)
    }
}

struct TweenData {
    start_pos: Point,
    end_pos: Point
}

pub struct CreatureAnimation {
    animation_type: AnimationType,
    time_progress: TimeProgress,
    looping: bool,
    tween_data: Option<TweenData>,
    at_end: Option<fn(&mut CreatureStack)>
}

fn turn_creature(creature: &mut CreatureStack) {
    creature.direction = creature.direction.inversion();
}

impl CreatureAnimation {
    pub fn new(animation_type: AnimationType) -> Self {
        Self::_new(animation_type, false, None, None, None)
    }
    pub fn new_looping(animation_type: AnimationType) -> Self {
        Self::_new(animation_type, true, None, None, None)
    }
    pub fn new_delayed(animation_type: AnimationType, delay: Duration) -> Self {
        Self::_new(animation_type, false, Some(delay), None, None)
    }
    pub fn new_turning(animation_type: AnimationType) -> Self {
        Self::_new(animation_type, false, None, None, Some(turn_creature))
    }
    pub fn new_tweening(start_pos: Point, end_pos: Point) -> Self {
        let tween_data = TweenData { start_pos, end_pos };
        Self::_new(AnimationType::Moving, false, None, Some(tween_data), None)
    }
    fn _new(
        animation_type: AnimationType,
        looping: bool,
        delay: Option<Duration>,
        tween_data: Option<TweenData>,
        at_end: Option<fn(&mut CreatureStack)>
    ) -> Self {
        Self {
            animation_type,
            time_progress: TimeProgress::new(animation_type.duration(), delay),
            looping,
            tween_data,
            at_end
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

    pub fn tween_data(&self) -> Option<(Point, Point)> {
        if let Some(tween_data) = &self.tween_data {
            let TweenData { start_pos, end_pos } = tween_data;
            Some((*start_pos, *end_pos))
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

    pub fn at_end(&self) -> Option<fn(&mut CreatureStack)> {
        self.at_end
    }

    pub fn total_duration(&self) -> Duration {
        if self.looping {
            Duration::from_secs(0)
        } else {
            self.time_progress.total_duration()
        }
    }
}