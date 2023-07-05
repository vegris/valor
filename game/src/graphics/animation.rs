use std::{collections::VecDeque, time::Duration};

use gamedata::Creature;

use crate::registry::ResourceRegistry;

use super::spritesheet::creature::AnimationType;

// Idle animation
// 10 frames per second
// 10 frames per 1000 ms
// 1 frame per 100 ms

#[derive(Clone, Copy, Debug)]
pub struct Anim {
    pub type_: AnimationType,
    duration: Duration,
    delay: Duration,
    spent: Duration,
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
}

impl Anim {
    pub fn new(
        animation_type: AnimationType,
        creature: Creature,
        rr: &mut ResourceRegistry,
    ) -> Self {
        let spritesheet = rr.get_creature_container(creature);
        let frames = spritesheet.frames_count(animation_type).unwrap();
        let duration = Duration::from_millis(150) * frames as u32;

        Self {
            type_: animation_type,
            duration,
            delay: Duration::ZERO,
            spent: Duration::ZERO,
        }
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
        }
    }

    pub fn push(&mut self, animation: Anim) {
        self.queue.push_back(animation);
    }

    pub fn update(&mut self, dt: Duration, creature: Creature, rr: &mut ResourceRegistry) {
        if let Some(anim) = self.queue.front_mut() {
            if let Status::Finished = anim.status() {
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

    pub fn get_animation(&self) -> (AnimationType, f32) {
        let idle = self.idle.unwrap();

        self.queue
            .front()
            .and_then(|anim| {
                let status = anim.status();
                if let Status::Delayed = status {
                    None
                } else {
                    Some((anim.type_, status.progress()))
                }
            })
            .unwrap_or((idle.type_, idle.status().progress()))
    }
}
