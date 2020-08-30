use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::rect::Point;

use crate::gamestate::creature::CreatureStack;
use crate::graphics::creature::AnimationType;


struct TimeProgress {
    start: Instant,
    end: Instant
}

enum TimeProgressState {
    Ready,
    Going(f32),
    Finished
}

impl TimeProgress{
    pub fn new(start: Instant, duration: Duration) -> Self {
        Self {
            start,
            end: start + duration
        }
    }

    pub fn state(&self, now: Instant) -> TimeProgressState {
        if now < self.start {
            TimeProgressState::Ready
        } else if now >= self.end {
            TimeProgressState::Finished
        } else {
            let progress = (now - self.start).as_secs_f32() / (self.end - self.start).as_secs_f32();
            TimeProgressState::Going(progress)
        }
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

impl CreatureAnimation {
    pub fn new_ordinary(animation_type: AnimationType, start_from: Instant) -> Self {
        let time_progress = TimeProgress::new(start_from, BASE_DURATION);
        Self {
            at_start: None,
            at_end: None,
            tween_data: None,
            animation_type,
            time_progress
        }
    }

    pub fn new_tweening(start_from: Instant, start_pos: Point, end_pos: Point) -> Self {
        let time_progress = TimeProgress::new(start_from, BASE_DURATION);
        let tween_data = TweenData {
            start_pos,
            end_pos
        };

        Self {
            at_start: None,
            at_end: None,
            tween_data: Some(tween_data),
            animation_type: AnimationType::Moving,
            time_progress
        }
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
    pub fn update(&self, creature: &mut CreatureStack, now: Instant) {
        if let TimeProgressState::Going(progress_percent) = self.time_progress.state(now) {
            // animation
            creature.set_animation_progress(progress_percent);

            // tweening
            if let Some(TweenData{ start_pos, end_pos }) = self.tween_data {
                let diff = end_pos - start_pos;
                let (diff_x, diff_y) = (diff.x() as f32, diff.y() as f32);

                let offset_x = (diff_x * progress_percent).round() as i32;
                let offset_y = (diff_y * progress_percent).round() as i32;

                let new_pos = start_pos.offset(offset_x, offset_y);
                creature.set_current_pos(new_pos);
            }
        }
    }
    pub fn is_finished(&self, now: Instant) -> bool {
        matches!(self.time_progress.state(now), TimeProgressState::Finished)
    }
    pub fn end(&self) -> Instant {
        self.time_progress.end
    }
}