use std::time::Duration;

extern crate sdl2;
use sdl2::rect::Point;

use crate::gamestate::creature::CreatureStack;
use crate::graphics::creature::AnimationType;


struct TimeProgress {
    delay: Option<Duration>,
    total_duration: Duration,
    time_passed: Duration
}

enum TimeProgressState {
    Ready,
    Going(f32),
    Finished
}

impl TimeProgress{
    pub fn new(duration: Duration) -> Self {
        Self {
            delay: None,
            total_duration: duration,
            time_passed: Duration::from_secs(0)
        }
    }

    pub fn new_delayed(duration: Duration, delay: Duration) -> Self {
        Self {
            delay: Some(delay),
            total_duration: duration,
            time_passed: Duration::from_secs(0)
        }
    }

    pub fn update(&mut self, dt: Duration) {
        if let Some(delay) = self.delay {
            self.delay = delay.checked_sub(dt);
        } else {
            self.time_passed += dt;
        }
    }

    pub fn progress(&self) -> TimeProgressState {
        if self.delay.is_some() {
            TimeProgressState::Ready
        } else if self.time_passed < self.total_duration {
            let progress = self.time_passed.as_secs_f32() / self.total_duration.as_secs_f32();
            TimeProgressState::Going(progress)
        } else {
            TimeProgressState::Finished
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.progress(), TimeProgressState::Finished)
    }
}

const BASE_DURATION: Duration = Duration::from_millis(435);

struct TweenData {
    start_pos: Point,
    end_pos: Point
}

type MutateCreatureStackFunc = fn(&mut CreatureStack);

pub struct CreatureAnimation {
    at_start: Option<MutateCreatureStackFunc>,
    at_end: Option<MutateCreatureStackFunc>,
    tween_data: Option<TweenData>,
    animation_type: AnimationType,
    time_progress: TimeProgress
}

fn repeat_animation(creature: &mut CreatureStack) {
    creature.push_animation(CreatureAnimation::new_looping());
}
fn turn_creature(creature: &mut CreatureStack) {
    creature.face_left = !creature.face_left;
}

impl CreatureAnimation {
    pub fn new(animation_type: AnimationType) -> Self {
        let time_progress = TimeProgress::new(BASE_DURATION);
        Self {
            at_start: None,
            at_end: None,
            tween_data: None,
            animation_type,
            time_progress
        }
    }

    pub fn new_delayed(animation_type: AnimationType, delay: Duration) -> Self {
        let time_progress = TimeProgress::new_delayed(BASE_DURATION, delay);
        Self {
            at_start: None,
            at_end: None,
            tween_data: None,
            animation_type,
            time_progress
        }
    }

    pub fn new_tweening(start_pos: Point, end_pos: Point) -> Self {
        let mut animation = Self::new(AnimationType::Moving);
        let tween_data = TweenData {
            start_pos,
            end_pos
        };
        animation.tween_data = Some(tween_data);
        animation
    }

    pub fn new_looping() -> Self {
        let mut animation = Self::new(AnimationType::Standing);
        animation.at_end = Some(repeat_animation);
        animation
    }

    pub fn new_turning() -> Self {
        let mut animation = Self::new(AnimationType::TurnRight);
        animation.at_start = Some(turn_creature);
        animation
    }

    pub fn at_start(&self, creature: &mut CreatureStack) {
        creature.set_animation_type(self.animation_type);
        if let Some(at_start) = self.at_start {
           at_start(creature);
        }
    }
    pub fn at_end(&self, creature: &mut CreatureStack) {
        if let Some(at_end) = self.at_end {
           at_end(creature);
        }
    }
    pub fn update(&mut self, creature: &mut CreatureStack, dt: Duration) {
        self.time_progress.update(dt);
        if let TimeProgressState::Going(progress) = self.time_progress.progress() {
            // animation
            creature.set_animation_progress(progress);

            // tweening
            if let Some(TweenData{ start_pos, end_pos }) = self.tween_data {
                let diff = end_pos - start_pos;
                let (diff_x, diff_y) = (diff.x() as f32, diff.y() as f32);

                let offset_x = (diff_x * progress).round() as i32;
                let offset_y = (diff_y * progress).round() as i32;

                let new_pos = start_pos.offset(offset_x, offset_y);
                creature.set_current_pos(new_pos);
            }
        }
    }
    pub fn is_finished(&self) -> bool {
        self.time_progress.is_finished()
    }
}