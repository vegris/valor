use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::rect::Point;

use crate::resources::AnimationType;


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
        } else if now > self.end {
            TimeProgressState::Finished
        } else {
            let progress = (now - self.start).as_secs_f32() / (self.end - self.start).as_secs_f32();
            TimeProgressState::Going(progress)
        }
    }
}


pub struct Tweening {
    current_pos: Point,
    next_pos: Point,
    time_progress: TimeProgress
}

impl Tweening {
    const TWEEN_DURATION: Duration = Duration::from_secs(1);

    pub fn new(current_pos: Point, next_pos: Point, start_from: Instant) -> Self {
        let time_progress = TimeProgress::new(start_from, Self::TWEEN_DURATION);
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

            let offset_x = (diff_x * progress_percent).floor() as i32;
            let offset_y = (diff_y * progress_percent).floor() as i32;

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
    is_looping: bool,
    loop_until: Option<Instant>
}

impl Animation {
    const LOOP_DURATION: Duration = Duration::from_millis(700);

    pub fn new_looping(type_: AnimationType, start_from: Instant, loop_until: Option<Instant>) -> Self {
        let time_progress = TimeProgress::new(start_from, Self::LOOP_DURATION);
        Self {
            type_,
            time_progress,
            is_looping: true,
            loop_until
        }
    }

    pub fn update(&mut self, now: Instant, type_: &mut AnimationType, progress: &mut f32) {
        match self.time_progress.state(now) {
            TimeProgressState::Going(progress_percent) => {
                *type_ = self.type_;
                *progress = progress_percent;
            },
            TimeProgressState::Finished => {
                if self.is_looping {
                    let new_loop_required = self.loop_until.map_or(true, |instant| instant > now);
                    if new_loop_required {
                        self.time_progress = TimeProgress::new(now, Self::LOOP_DURATION);
                    }
                }
            },
            TimeProgressState::Ready => {}
        }
    }

    pub fn end(&self) -> Instant {
        self.time_progress.end
    }

    pub fn is_finished(&self, now: Instant) -> bool {
        matches!(self.time_progress.state(now), TimeProgressState::Finished)
    }
}
