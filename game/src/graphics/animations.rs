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

pub struct AnimationState {
    creature: Creature,
    event_queue: VecDeque<AnimationEvent>,
    idle: Animation,
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
        let idle = Animation::new(AnimationType::Standing, creature, rr);

        Self {
            creature,
            event_queue: VecDeque::new(),
            idle,
            invert_side: false,
            position: position.center(),
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        while let Some(event) = self.event_queue.front_mut() {
            match event {
                AnimationEvent::Instant(instant_event) => {
                    match instant_event {
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
                    self.event_queue.pop_front();
                }
                AnimationEvent::TimeProgress(progress_event) => {
                    if progress_event.as_ref().is_finished() {
                        self.event_queue.pop_front();
                    } else {
                        progress_event.as_mut().update(dt);

                        if let TimeProgressEvent::Movement(movement) = progress_event {
                            self.position = movement.get_position();
                        }

                        break;
                    }
                }
            }
        }

        let idle_progress = self.idle.progress_mut();

        if idle_progress.is_finished() {
            idle_progress.reset();
        }
        idle_progress.update(dt);
    }

    pub fn get_state(&self) -> AnimationData {
        let (animation_type, frame_index) = self
            .event_queue
            .front()
            .and_then(|event| match event {
                AnimationEvent::Instant(_) => None,
                AnimationEvent::TimeProgress(progress_event) => match progress_event {
                    TimeProgressEvent::Animation(animation) => {
                        Some((animation.type_, animation.get_frame()))
                    }
                    TimeProgressEvent::Movement(movement) => {
                        Some((Movement::ANIMATION_TYPE, movement.get_frame()))
                    }
                    _ => None,
                },
            })
            .unwrap_or((self.idle.type_, self.idle.get_frame()));

        AnimationData {
            type_: animation_type,
            frame_index,
            invert_side: self.invert_side,
            position: self.position,
        }
    }

    pub fn total_duration(&self) -> Duration {
        self.event_queue
            .iter()
            .map(|event| match event {
                AnimationEvent::Instant(_) => Duration::ZERO,
                AnimationEvent::TimeProgress(progress_event) => progress_event.as_ref().time_left(),
            })
            .sum()
    }

    pub fn is_animating(&self) -> bool {
        !self.event_queue.is_empty()
    }

    fn put_event(&mut self, event: AnimationEvent) {
        self.event_queue.push_back(event);
    }
}
