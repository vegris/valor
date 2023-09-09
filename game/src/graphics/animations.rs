use std::collections::VecDeque;
use std::time::Duration;

use sdl2::rect::Point;

use gamedata::creatures::Creature;

use crate::battlestate::BattleState;
use crate::event::Event;
use crate::grid::GridPos;
use crate::registry::ResourceRegistry;
use crate::sound;

use super::creature::AnimationType;
use super::Animations;

mod animation;
mod choreographer;
mod event;
mod movement;
mod time_progress;

use self::animation::Animation;
use self::event::{AnimationEvent, InstantEvent, TimeProgressEvent};
use self::movement::Movement;
use self::time_progress::TimeProgress;

pub struct AnimationState {
    creature: Creature,
    event_queue: VecDeque<AnimationEvent>,
    current_event: CurrentEvent,
    invert_side: bool,
    pub position: Point,
}

enum CurrentEvent {
    Event(TimeProgressEvent),
    Idle(Idle),
}

struct Idle {
    animation: Animation,
    delay: TimeProgress,
}

pub struct AnimationData {
    pub type_: AnimationType,
    pub frame_index: usize,
    pub invert_side: bool,
    pub position: Point,
}

pub fn process_event(
    state: &BattleState,
    event: Event,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    match event {
        Event::Attack(attack) => choreographer::animate_attack(attack, state, animations, rr),
        Event::Shot(shot) => choreographer::animate_shot(shot, state, animations, rr),
        Event::Movement(movement) => {
            choreographer::animate_movement(movement, state, animations, rr)
        }
    }
}

impl AnimationState {
    pub fn new(creature: Creature, position: GridPos, rr: &mut ResourceRegistry) -> Self {
        let idle = Animation::new(AnimationType::Standing, creature, rr);

        Self {
            creature,
            event_queue: VecDeque::new(),
            current_event: CurrentEvent::Idle(Idle {
                animation: idle,
                delay: TimeProgress::new(Duration::ZERO),
            }),
            invert_side: false,
            position: position.center(),
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        let current_event_finished = match &self.current_event {
            CurrentEvent::Event(event) => event.as_ref(),
            CurrentEvent::Idle(idle) => &idle.delay,
        }
        .is_finished();

        if current_event_finished {
            while let Some(event) = self.event_queue.pop_front() {
                match event {
                    AnimationEvent::Instant(instant_event) => match instant_event {
                        InstantEvent::InvertSide => {
                            self.invert_side = !self.invert_side;
                        }
                        InstantEvent::PlaySound(sound) => {
                            if let Some(chunk) = rr.get_creature_sound(self.creature, sound.type_) {
                                sound::play_sound(chunk, sound.looping).unwrap();
                            }
                        }
                        InstantEvent::StopSound => {
                            sound::stop_looping();
                        }
                        InstantEvent::Teleport(position) => {
                            self.position = position.center();
                        }
                    },
                    AnimationEvent::TimeProgress(progress_event) => {
                        self.current_event = CurrentEvent::Event(progress_event);
                        break;
                    }
                    AnimationEvent::Delay(delay) => {
                        match self.current_event {
                            CurrentEvent::Event(_) => {
                                let idle =
                                    Animation::new(AnimationType::Standing, self.creature, rr);
                                self.current_event = CurrentEvent::Idle(Idle {
                                    animation: idle,
                                    delay,
                                });
                            }
                            CurrentEvent::Idle(ref mut idle) => {
                                idle.delay.duration += delay.duration;
                            }
                        }
                        break;
                    }
                }
            }
        }

        match self.current_event {
            CurrentEvent::Event(ref mut event) => {
                if let TimeProgressEvent::Movement(movement) = event {
                    self.position = movement.get_position();
                }

                let event_progress = event.as_mut();
                if event_progress.is_finished() {
                    let idle = Animation::new(AnimationType::Standing, self.creature, rr);
                    self.current_event = CurrentEvent::Idle(Idle {
                        animation: idle,
                        delay: TimeProgress::new(Duration::ZERO),
                    });
                } else {
                    event_progress.update(dt);
                }
            }
            CurrentEvent::Idle(ref mut idle) => {
                if !idle.delay.is_finished() {
                    idle.delay.update(dt);
                }

                let idle_progress = idle.animation.as_mut();
                if idle_progress.is_finished() {
                    idle_progress.reset();
                }
                idle_progress.update(dt);
            }
        }
    }

    pub fn get_state(&self) -> AnimationData {
        let (animation_type, frame_index) = match &self.current_event {
            CurrentEvent::Event(event) => match event {
                TimeProgressEvent::Animation(animation) => (animation.type_, animation.get_frame()),
                TimeProgressEvent::Movement(movement) => {
                    (Movement::ANIMATION_TYPE, movement.get_frame())
                }
            },
            CurrentEvent::Idle(idle) => (idle.animation.type_, idle.animation.get_frame()),
        };

        AnimationData {
            type_: animation_type,
            frame_index,
            invert_side: self.invert_side,
            position: self.position,
        }
    }

    pub fn total_duration(&self) -> Duration {
        let current_duration = match &self.current_event {
            CurrentEvent::Event(event) => event.as_ref().time_left(),
            CurrentEvent::Idle(idle) => idle.delay.time_left(),
        };

        let queue_duration = self
            .event_queue
            .iter()
            .map(|event| match event {
                AnimationEvent::Instant(_) => Duration::ZERO,
                AnimationEvent::TimeProgress(progress_event) => progress_event.as_ref().time_left(),
                AnimationEvent::Delay(progress) => progress.time_left(),
            })
            .sum();

        current_duration + queue_duration
    }

    pub fn is_animating(&self) -> bool {
        !self.event_queue.is_empty()
    }

    fn put_event(&mut self, event: AnimationEvent) {
        self.event_queue.push_back(event);
    }
}
