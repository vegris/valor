use std::collections::VecDeque;
use std::time::Duration;

use gamedata::creatures::sounds::CreatureSound;

use gamedata::creatures::Creature;
use sdl2::rect::Point;

use crate::battlestate::BattleState;
use crate::event::Event;
use crate::grid::GridPos;
use crate::registry::ResourceRegistry;
use crate::sound;

use super::creature::AnimationType;
use super::Animations;

mod animation;
mod choreographer;
mod movement;
mod time_progress;

use self::animation::Animation;
use self::movement::Movement;
use self::time_progress::TimeProgress;

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

enum AnimationEvent {
    Animation(Animation),
    Delay(TimeProgress),
    InvertSide,
    PlaySound(Sound),
    StopSound,
    Movement(Movement),
    Teleport(GridPos),
}

struct Sound {
    type_: CreatureSound,
    looping: bool,
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
                    if let Some(chunk) = rr.get_creature_sound(self.creature, sound.type_) {
                        sound::play_sound(chunk, sound.looping).unwrap();
                    }
                    update_result.event_finished = true;
                }
                AnimationEvent::StopSound => {
                    sound::stop_looping();
                    update_result.event_finished = true;
                }
                AnimationEvent::Movement(movement) => {
                    update_progress(&mut movement.progress, dt, &mut update_result);

                    if !update_result.event_finished {
                        self.position = movement.get_position();
                    }

                    if update_result.consumed_dt {
                        animation_in_progress = true;
                    };
                }
                AnimationEvent::Teleport(pos) => {
                    self.position = pos.center();
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
        let (animation_type, frame_index) = self
            .event_queue
            .front()
            .and_then(|event| match event {
                AnimationEvent::Animation(animation) => {
                    Some((animation.type_, animation.get_frame()))
                }
                AnimationEvent::Movement(movement) => {
                    Some((Movement::ANIMATION_TYPE, movement.get_frame()))
                }
                _ => None,
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
                AnimationEvent::Animation(animation) => animation.progress.time_left(),
                AnimationEvent::Delay(progress) => progress.time_left(),
                AnimationEvent::InvertSide => Duration::ZERO,
                AnimationEvent::PlaySound(_) => Duration::ZERO,
                AnimationEvent::StopSound => Duration::ZERO,
                AnimationEvent::Movement(movement) => movement.progress.time_left(),
                AnimationEvent::Teleport(_) => Duration::ZERO,
            })
            .sum()
    }

    pub fn is_animating(&self) -> bool {
        !self.event_queue.is_empty()
    }

    fn put_animation(
        &mut self,
        animation_type: AnimationType,
        creature: Creature,
        rr: &mut ResourceRegistry,
    ) {
        self.put_sound(animation_type);

        let animation = Animation::new(animation_type, creature, rr);
        let event = AnimationEvent::Animation(animation);
        self.event_queue.push_back(event);
    }

    fn put_delay(&mut self, duration: Duration) {
        let progress = TimeProgress::new(duration);
        let event = AnimationEvent::Delay(progress);
        self.event_queue.push_back(event);
    }

    fn put_movement(&mut self, creature: Creature, path: Vec<GridPos>, rr: &mut ResourceRegistry) {
        self.event_queue.push_back(AnimationEvent::PlaySound(Sound {
            type_: CreatureSound::StartMoving,
            looping: false,
        }));

        let animation_type = AnimationType::StartMoving;
        if rr
            .get_creature_spritesheet(creature)
            .frames_count(animation_type)
            .is_some()
        {
            let animation = Animation::new(animation_type, creature, rr);
            self.event_queue
                .push_back(AnimationEvent::Animation(animation));
        }

        if creature.is_teleporting() {
            self.event_queue
                .push_back(AnimationEvent::Teleport(*path.last().unwrap()));
        } else {
            self.event_queue.push_back(AnimationEvent::PlaySound(Sound {
                type_: CreatureSound::Move,
                looping: true,
            }));

            let movement = Movement::new(creature, path, rr);
            self.event_queue
                .push_back(AnimationEvent::Movement(movement));

            self.event_queue.push_back(AnimationEvent::StopSound);
        }

        self.event_queue.push_back(AnimationEvent::PlaySound(Sound {
            type_: CreatureSound::EndMoving,
            looping: false,
        }));

        let animation_type = AnimationType::StopMoving;
        if rr
            .get_creature_spritesheet(creature)
            .frames_count(animation_type)
            .is_some()
        {
            let animation = Animation::new(animation_type, creature, rr);
            self.event_queue
                .push_back(AnimationEvent::Animation(animation));
        }
    }

    fn put_event(&mut self, event: AnimationEvent) {
        self.event_queue.push_back(event);
    }

    fn put_sound(&mut self, animation_type: AnimationType) {
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
            self.event_queue.push_back(AnimationEvent::PlaySound(Sound {
                type_: sound_type,
                looping: false,
            }));
        }
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
