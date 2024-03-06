use std::collections::VecDeque;
use std::time::Duration;

use sdl2::rect::Point;

use gamedata::creatures::Creature;

use logic::event::Event;
use logic::gamestate::GameState;
use logic::grid::GridPos;

use crate::registry::ResourceRegistry;
use crate::sound;

use super::creature::AnimationType;
use super::Animations;

mod animation;
mod choreographer;
mod current_event;
pub mod entity_animations;
mod events;
mod movement;
mod time_progress;

use self::current_event::{CurrentEvent, Idle};
use self::entity_animations::EntityAnimations;
use self::events::{
    AnimationEvent, AnimationEventByGroup, InstantEvent, TimeEvent, TimeProgressEvent,
};

pub struct AnimationState {
    creature: Creature,
    event_queue: VecDeque<AnimationEventByGroup>,
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
    state: &GameState,
    event: Event,
    animations: &mut Animations,
    entity_animations: &mut EntityAnimations,
    rr: &mut ResourceRegistry,
) {
    match event {
        Event::Attack(attack) => choreographer::animate_attack(attack, state, animations, rr),
        Event::Shot(shot) => choreographer::animate_shot(shot, state, animations, rr),
        Event::Movement(movement) => {
            choreographer::animate_movement(movement, state, animations, rr)
        }
        Event::Cast(cast) => {
            choreographer::animate_cast(cast, state, animations, entity_animations, rr)
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
        } else if !matches!(self.current_event, CurrentEvent::Idle(_)) {
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
                AnimationEventByGroup::Instant(_) => Duration::ZERO,
                AnimationEventByGroup::Time(time_event) => match time_event {
                    TimeEvent::Delay(duration) => *duration,
                    TimeEvent::TimeProgress(progress_event) => {
                        progress_event.progress().time_left()
                    }
                },
            })
            .sum();

        self.current_event.time_left() + queue_duration
    }

    pub fn is_animating(&self) -> bool {
        !self.event_queue.is_empty()
    }

    fn push_event(&mut self, event: AnimationEvent) {
        self.event_queue.push_back(event.into());
    }
}

fn find_time_progress_event(
    event_queue: &mut VecDeque<AnimationEventByGroup>,
) -> (Box<[InstantEvent]>, Option<TimeEvent>) {
    let mut events = vec![];

    while let Some(event) = event_queue.pop_front() {
        match event {
            AnimationEventByGroup::Instant(instant_event) => events.push(instant_event),
            AnimationEventByGroup::Time(time_event) => return (events.into(), Some(time_event)),
        }
    }

    (events.into(), None)
}
