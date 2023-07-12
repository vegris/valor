use std::{collections::VecDeque, time::Duration};

use sdl2::rect::Point;

use gamedata::creatures::Creature;

use crate::registry::ResourceRegistry;

use super::spritesheet::creature::AnimationType;

// Idle animation
// 10 frames per second
// 10 frames per 1000 ms
// 1 frame per 100 ms

#[derive(Clone, Copy, Debug)]
struct Tween {
    from: Point,
    to: Point,
}

#[derive(Clone, Copy, Debug)]
pub struct Anim {
    pub type_: AnimationType,
    duration: Duration,
    delay: Duration,
    spent: Duration,
    at_end: Option<AtEndEvent>,
    tween: Option<Tween>,
}

#[derive(Clone, Copy, Debug)]
pub enum AtEndEvent {
    InvertSide,
}

pub enum Status {
    Delayed,
    Progress(f32),
    Finished,
}

#[derive(Clone, Debug)]
pub struct AnimationQueue {
    queue: VecDeque<Anim>,
    idle: Option<Anim>,
    invert_side: bool,
}

impl Anim {
    pub fn new(
        animation_type: AnimationType,
        creature: Creature,
        rr: &mut ResourceRegistry,
    ) -> Self {
        let spritesheet = rr.get_creature_container(creature);

        let frames = spritesheet.frames_count(animation_type).unwrap();
        let duration = frame_duration(animation_type) * frames as u32;

        Self {
            type_: animation_type,
            duration,
            delay: Duration::ZERO,
            spent: Duration::ZERO,
            at_end: None,
            tween: None,
        }
    }

    pub fn add_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn set_at_end(mut self, at_end: AtEndEvent) -> Self {
        self.at_end = Some(at_end);
        self
    }

    pub fn add_tween(mut self, from: Point, to: Point) -> Self {
        self.tween = Some(Tween { from, to });
        self
    }

    pub fn update(&mut self, dt: Duration) {
        if self.delay >= dt {
            self.delay -= dt;
        } else {
            self.spent += dt - self.delay;
            self.delay = Duration::ZERO;
        }
    }

    pub fn status(&self) -> Status {
        if !self.delay.is_zero() {
            return Status::Delayed;
        }

        if self.spent >= self.duration {
            return Status::Finished;
        }

        let progress = self.spent.as_secs_f32() / self.duration.as_secs_f32();
        Status::Progress(progress)
    }

    pub fn duration(&self) -> Duration {
        self.delay + self.duration - self.spent
    }

    pub fn tween(&self) -> Option<Point> {
        if let Some(tween) = self.tween {
            let progress = self.status().progress();
            let x = tween.from.x + ((tween.to.x - tween.from.x) as f32 * progress) as i32;
            let y = tween.from.y + ((tween.to.y - tween.from.y) as f32 * progress) as i32;
            Some(Point::new(x, y))
        } else {
            None
        }
    }
}

impl Status {
    pub fn progress(&self) -> f32 {
        match self {
            Self::Delayed => unreachable!(),
            Self::Progress(progress) => *progress,
            Self::Finished => 1.0,
        }
    }
}

impl AnimationQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            idle: None,
            invert_side: false,
        }
    }

    pub fn push(&mut self, animation: Anim) {
        self.queue.push_back(animation);
    }

    pub fn update(&mut self, dt: Duration, creature: Creature, rr: &mut ResourceRegistry) {
        if let Some(anim) = self.queue.front_mut() {
            if let Status::Finished = anim.status() {
                if let Some(at_end) = anim.at_end {
                    match at_end {
                        AtEndEvent::InvertSide => self.invert_side = !self.invert_side,
                    }
                }
                self.queue.pop_front();
            }
        }

        if let Some(anim) = self.queue.front_mut() {
            anim.update(dt);
            return;
        }

        if let Some(idle) = &mut self.idle {
            if let Status::Finished = idle.status() {
                self.idle = Some(Anim::new(AnimationType::Standing, creature, rr));
            } else {
                idle.update(dt);
            }
        } else {
            self.idle = Some(Anim::new(AnimationType::Standing, creature, rr));
        }
    }

    pub fn get_animation(&self) -> (AnimationType, f32, bool, Option<Point>) {
        let idle = self.idle.unwrap();

        self.queue
            .front()
            .and_then(|anim| {
                let status = anim.status();
                if let Status::Delayed = status {
                    None
                } else {
                    Some((
                        anim.type_,
                        status.progress(),
                        self.invert_side,
                        anim.tween(),
                    ))
                }
            })
            .unwrap_or((idle.type_, idle.status().progress(), self.invert_side, None))
    }

    pub fn total_duration(&self) -> Duration {
        self.queue
            .iter()
            .map(|animation| animation.duration())
            .sum()
    }

    pub fn is_animating(&self) -> bool {
        self.queue
            .iter()
            .any(|animation| animation.type_ != AnimationType::Standing)
    }
}

fn frame_duration(animation_type: AnimationType) -> Duration {
    let ms = match animation_type {
        AnimationType::Standing => 200,
        AnimationType::TurnLeft | AnimationType::TurnRight => 100,
        AnimationType::Moving => 50,
        _ => 100,
    };

    Duration::from_millis(ms)
}
