use std::time::Duration;

use gamedata::creatures;
use gamedata::creatures::Creature;

use crate::resources::ResourceRegistry;

use super::animation::Animation;
use super::events::TimeProgressEvent;
use super::movement::Movement;
use super::time_progress::TimeProgress;

pub enum CurrentEvent {
    Event(TimeProgressEvent),
    Idle(Idle),
}

pub struct Idle {
    animation: Animation,
    delay: Option<TimeProgress>,
}

impl CurrentEvent {
    pub fn is_finished(&self) -> bool {
        match self {
            Self::Event(event) => event.progress().is_finished(),
            Self::Idle(idle) => idle.is_finished(),
        }
    }

    pub fn time_left(&self) -> Duration {
        match self {
            Self::Event(event) => event.progress().time_left(),
            Self::Idle(idle) => idle.time_left(),
        }
    }

    pub fn animation_state(&self) -> (creatures::Animation, usize) {
        match self {
            CurrentEvent::Event(event) => match event {
                TimeProgressEvent::Animation(animation) => (animation.type_, animation.get_frame()),
                TimeProgressEvent::Movement(movement) => {
                    (Movement::ANIMATION_TYPE, movement.get_frame())
                }
            },
            CurrentEvent::Idle(idle) => (idle.animation.type_, idle.animation.get_frame()),
        }
    }
}

impl Idle {
    pub fn empty(creature: Creature, rr: &mut ResourceRegistry) -> Self {
        Self::_new(creature, rr, None)
    }

    pub fn new(creature: Creature, rr: &mut ResourceRegistry, delay: Duration) -> Self {
        Self::_new(creature, rr, Some(delay))
    }

    fn _new(creature: Creature, rr: &mut ResourceRegistry, delay: Option<Duration>) -> Self {
        Self {
            animation: Animation::new(creatures::Animation::Standing, creature, rr),
            delay: delay.map(TimeProgress::new),
        }
    }

    pub fn update_animation(&mut self, dt: Duration) {
        let animation_progress = self.animation.progress_mut();

        if animation_progress.is_finished() {
            animation_progress.reset();
        } else {
            animation_progress.update(dt);
        }
    }

    pub fn update_delay(&mut self, dt: Duration) {
        if let Some(ref mut progress) = self.delay {
            if progress.is_finished() {
                self.delay = None;
            } else {
                progress.update(dt);
            }
        }
    }

    fn time_left(&self) -> Duration {
        self.delay
            .as_ref()
            .map_or(Duration::ZERO, |progress| progress.time_left())
    }

    fn is_finished(&self) -> bool {
        self.delay
            .as_ref()
            .map_or(true, |progress| progress.is_finished())
    }
}
