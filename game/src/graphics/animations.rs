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
mod current_event;
mod event;
mod movement;
mod time_progress;

use self::current_event::{CurrentEvent, Idle};
use self::event::{AnimationEvent, InstantEvent, TimeProgressEvent};

pub struct AnimationState {
    creature: Creature,
    event_queue: VecDeque<AnimationEvent>,
    current_event: CurrentEvent,
    invert_side: bool,
    pub position: Point,
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
        Self {
            creature,
            event_queue: VecDeque::new(),
            current_event: CurrentEvent::Idle(Idle::empty(creature, rr)),
            invert_side: false,
            position: position.center(),
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        if let CurrentEvent::Idle(ref mut idle) = self.current_event {
            idle.update_animation(dt);
        }

        if !self.current_event.is_finished() {
            match self.current_event {
                CurrentEvent::Event(ref mut event) => {
                    if let TimeProgressEvent::Movement(movement) = event {
                        self.position = movement.get_position();
                    }

                    event.progress_mut().update(dt);
                }
                CurrentEvent::Idle(ref mut idle) => {
                    idle.update_delay(dt);
                }
            }
            return;
        }

        let (instant_events, time_event) = find_time_progress_event(&mut self.event_queue);

        for event in instant_events.iter() {
            match event {
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
            }
        }

        if let Some(event) = time_event {
            match event {
                TimeEvent::Delay(duration) => {
                    self.current_event = CurrentEvent::Idle(Idle::new(self.creature, rr, duration));
                }
                TimeEvent::TimeProgress(progress_event) => {
                    self.current_event = CurrentEvent::Event(progress_event);
                }
            }
        } else {
            self.current_event = CurrentEvent::Idle(Idle::empty(self.creature, rr));
        }
    }

    pub fn get_state(&self) -> AnimationData {
        let (animation_type, frame_index) = self.current_event.animation_state();

        AnimationData {
            type_: animation_type,
            frame_index,
            invert_side: self.invert_side,
            position: self.position,
        }
    }

    pub fn total_duration(&self) -> Duration {
        let queue_duration = self
            .event_queue
            .iter()
            .map(|event| match event {
                AnimationEvent::Instant(_) => Duration::ZERO,
                AnimationEvent::TimeProgress(progress_event) => {
                    progress_event.progress().time_left()
                }
                AnimationEvent::Delay(duration) => *duration,
            })
            .sum();

        self.current_event.time_left() + queue_duration
    }

    pub fn is_animating(&self) -> bool {
        !self.event_queue.is_empty()
    }

    fn put_event(&mut self, event: AnimationEvent) {
        self.event_queue.push_back(event);
    }
}

enum TimeEvent {
    TimeProgress(TimeProgressEvent),
    Delay(Duration),
}

fn find_time_progress_event(
    event_queue: &mut VecDeque<AnimationEvent>,
) -> (Box<[InstantEvent]>, Option<TimeEvent>) {
    let mut events = vec![];
    let mut time_event = None;

    while let Some(event) = event_queue.pop_front() {
        match event {
            AnimationEvent::Instant(instant_event) => events.push(instant_event),
            AnimationEvent::Delay(duration) => {
                time_event = Some(TimeEvent::Delay(duration));
                break;
            }
            AnimationEvent::TimeProgress(progress_event) => {
                time_event = Some(TimeEvent::TimeProgress(progress_event));
                break;
            }
        }
    }

    (events.into_boxed_slice(), time_event)
}
