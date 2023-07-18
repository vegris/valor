use std::time::Duration;

pub struct TimeProgress {
    duration: Duration,
    spent: Duration,
}

impl TimeProgress {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            spent: Duration::ZERO,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.spent += dt;
    }

    pub fn progress(&self) -> f32 {
        let progress = self.spent.as_secs_f32() / self.duration.as_secs_f32();
        f32::min(progress, 1.0)
    }

    pub fn is_finished(&self) -> bool {
        self.spent >= self.duration
    }

    pub fn reset(&mut self) {
        self.spent = Duration::ZERO;
    }

    pub fn time_left(&self) -> Duration {
        self.duration - self.spent
    }
}
