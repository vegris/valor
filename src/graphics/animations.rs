use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::rect::Point;

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


pub struct Tweening {
    current_pos: Point,
    next_pos: Point,
    time_progress: TimeProgress
}

impl Tweening {
    pub fn new(current_pos: Point, next_pos: Point, start_from: Instant) -> Self {
        let time_progress = TimeProgress::new(start_from, BASE_DURATION * 2);
        Self {
            current_pos,
            next_pos,
            time_progress
        }
    }

    pub fn update(&self, now: Instant, pos: &mut Point)  {
        if let TimeProgressState::Going(progress_percent) = self.time_progress.state(now) {
            let diff = self.next_pos - self.current_pos;
            let (diff_x, diff_y) = (diff.x() as f32, diff.y() as f32);

            let offset_x = (diff_x * progress_percent).round() as i32;
            let offset_y = (diff_y * progress_percent).round() as i32;

            *pos = self.current_pos.offset(offset_x, offset_y);
        }
    }

    pub fn is_finished(&self, now: Instant) -> bool {
        matches!(self.time_progress.state(now), TimeProgressState::Finished)
    }

    pub fn end(&self) -> Instant {
        self.time_progress.end
    }
}


pub struct Animation {
    type_: AnimationType,
    time_progress: TimeProgress,
    is_looping: bool
}

impl Animation {

    fn get_duration(type_: AnimationType) -> Duration {
        match type_ {
            AnimationType::StartMoving | AnimationType::StopMoving => BASE_DURATION / 3,
            AnimationType::Moving => BASE_DURATION * 2,
            AnimationType::Standing => BASE_DURATION * 2,
            _ => BASE_DURATION
        }
    }

    pub fn new(type_: AnimationType, start_from: Instant) -> Self {
        Self::do_new(type_, start_from, false)
    }

    pub fn new_looping(type_: AnimationType, start_from: Instant) -> Self {
        Self::do_new(type_, start_from, true)
    }

    fn do_new(type_: AnimationType, start_from: Instant, is_looping: bool) -> Self {
        let time_progress = TimeProgress::new(start_from, Self::get_duration(type_));
        Self {
            type_,
            time_progress,
            is_looping
        }
    }

    pub fn update(&mut self, now: Instant, progress: &mut f32) {
        match self.time_progress.state(now) {
            TimeProgressState::Going(progress_percent) => {
                *progress = progress_percent;
            },
            TimeProgressState::Finished => {
                if self.is_looping {
                    self.time_progress = TimeProgress::new(now, Self::get_duration(self.type_));
                }
            },
            TimeProgressState::Ready => {}
        }
    }

    pub fn end(&self) -> Instant {
        self.time_progress.end
    }

    pub fn type_(&self) -> AnimationType {
        self.type_
    }

    pub fn is_finished(&self, now: Instant) -> bool {
        matches!(self.time_progress.state(now), TimeProgressState::Finished)
    }
}
