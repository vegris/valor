use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::rect::Point;


const TOTAL_TWEENING_DURATION: Duration = Duration::from_secs(1);


pub struct Tweening {
    start_pos: Point,
    end_pos: Point,
    start_time: Option<Instant>,
    cur_pos: Point,
    finished: bool
}

impl Tweening {
    pub fn new(start_pos: Point, end_pos: Point) -> Self {
        Self {
            start_pos,
            end_pos,
            start_time: None,
            cur_pos: start_pos,
            finished: false
        }
    }
    pub fn update(&mut self, now: Instant) {
        if self.finished { return };

        if let Some(start_time) = self.start_time {
            let elapsed = now.duration_since(start_time);
            if elapsed >= TOTAL_TWEENING_DURATION {
                self.cur_pos = self.end_pos;
                self.finished = true;
            } else {
                let percent_completion = elapsed.as_secs_f32() / TOTAL_TWEENING_DURATION.as_secs_f32();
                let diff_pos = self.end_pos - self.start_pos;
                let diff_x = (diff_pos.x() as f32 * percent_completion).round() as i32;
                let diff_y = (diff_pos.y() as f32 * percent_completion).round() as i32;
                self.cur_pos = self.start_pos.offset(diff_x, diff_y);
            }
        } else {
            self.start_time = Some(now);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn draw_point(&self) -> Point {
        self.cur_pos
    }
}