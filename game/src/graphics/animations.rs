use std::collections::VecDeque;
use std::time::Duration;

use gamedata::creatures::sounds::CreatureSound;
use sdl2::rect::Point;

use gamedata::creatures::Creature;

use crate::battlestate::BattleState;
use crate::event::Event;
use crate::registry::ResourceRegistry;

use super::spritesheet::creature::AnimationType;
use super::Animations;

mod animation;
mod choreographer;
mod time_progress;

use self::animation::Animation;
use self::time_progress::TimeProgress;

pub struct AnimationState {
    event_queue: VecDeque<AnimationEvent>,
    idle: Animation,
    invert_side: bool,
}

pub struct AnimationData {
    pub type_: AnimationType,
    pub frame_index: usize,
    pub position: Option<Point>,
    pub invert_side: bool,
}

enum AnimationEvent {
    Animation(Animation),
    Delay(TimeProgress),
    InvertSide,
    PlaySound(&'static str),
}

#[derive(Default)]
struct UpdateResult {
    event_finished: bool,
    consumed_dt: bool,
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
    pub fn new(creature: Creature, rr: &mut ResourceRegistry) -> Self {
        let idle = Animation::new(AnimationType::Standing, creature, rr);

        Self {
            event_queue: VecDeque::new(),
            idle,
            invert_side: false,
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        let mut animation_in_progress = false;

        while let Some(event) = self.event_queue.front_mut() {
            let mut update_result = UpdateResult::default();

            match event {
                AnimationEvent::Animation(animation) => {
                    update_progress(&mut animation.progress, dt, &mut update_result);

                    if update_result.consumed_dt {
                        animation_in_progress = true;
                    };
                }
                AnimationEvent::Delay(progress) => {
                    update_progress(progress, dt, &mut update_result);
                }
                AnimationEvent::InvertSide => {
                    self.invert_side = !self.invert_side;
                    update_result.event_finished = true;
                }
                AnimationEvent::PlaySound(sound) => {
                    let sound = rr.get_sound(sound);
                    let channel = sdl2::mixer::Channel(-1);
                    channel.play(sound, 0).unwrap();

                    update_result.event_finished = true;
                }
            }

            if update_result.event_finished {
                self.event_queue.pop_front();
            }

            if update_result.consumed_dt {
                break;
            }
        }

        if animation_in_progress {
            self.idle.progress.reset();
        } else {
            if self.idle.progress.is_finished() {
                self.idle.progress.reset();
            }
            self.idle.progress.update(dt);
        }
    }

    pub fn get_state(&self) -> AnimationData {
        let animation = self
            .event_queue
            .front()
            .and_then(|event| {
                if let AnimationEvent::Animation(animation) = event {
                    Some(animation)
                } else {
                    None
                }
            })
            .unwrap_or(&self.idle);

        let state = animation.get_state();

        AnimationData {
            type_: animation.type_,
            frame_index: state.frame_index,
            position: state.position,
            invert_side: self.invert_side,
        }
    }

    pub fn total_duration(&self) -> Duration {
        self.event_queue
            .iter()
            .map(|event| match event {
                AnimationEvent::Animation(animation) => animation.progress.time_left(),
                AnimationEvent::Delay(progress) => progress.time_left(),
                AnimationEvent::InvertSide => Duration::ZERO,
                AnimationEvent::PlaySound(_) => Duration::ZERO,
            })
            .sum()
    }

    fn put_animation(
        &mut self,
        animation_type: AnimationType,
        creature: Creature,
        rr: &mut ResourceRegistry,
    ) {
        let animation = Animation::new(animation_type, creature, rr);

        let sound_type = match animation_type {
            AnimationType::AttackStraight => Some(CreatureSound::Attack),
            AnimationType::Defend => Some(CreatureSound::Defend),
            AnimationType::StartMoving => Some(CreatureSound::StartMoving),
            AnimationType::Moving => Some(CreatureSound::Move),
            AnimationType::StopMoving => Some(CreatureSound::EndMoving),
            AnimationType::ShootStraight => Some(CreatureSound::Shoot),
            AnimationType::GettingHit => Some(CreatureSound::Wince),
            AnimationType::Death => Some(CreatureSound::Killed),
            _ => None,
        };

        if let Some(sound_type) = sound_type {
            if let Some(filename) = creature.sounds().get(sound_type) {
                self.event_queue
                    .push_back(AnimationEvent::PlaySound(filename));
            }
        }

        let event = AnimationEvent::Animation(animation);
        self.event_queue.push_back(event);
    }

    fn put_delay(&mut self, duration: Duration) {
        let progress = TimeProgress::new(duration);
        let event = AnimationEvent::Delay(progress);
        self.event_queue.push_back(event);
    }

    fn put_event(&mut self, event: AnimationEvent) {
        self.event_queue.push_back(event);
    }
}

fn update_progress(progress: &mut TimeProgress, dt: Duration, update_result: &mut UpdateResult) {
    if progress.is_finished() {
        update_result.event_finished = true;
    } else {
        progress.update(dt);
        update_result.consumed_dt = true;
    };
}
