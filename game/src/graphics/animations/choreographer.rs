use std::fmt::Debug;
use std::time::Duration;

use gamedata::creatures::sounds::CreatureSound;
use gamedata::creatures::Creature;

use crate::battlestate::{BattleState, Side, StackHandle};
use crate::event::{Attack, Movement, Shot};
use crate::graphics::creature::AnimationType;
use crate::graphics::Animations;
use crate::grid::GridPos;
use crate::registry::ResourceRegistry;
use crate::stack::Stack;

use super::animation::Animation;
use super::movement::Movement as MovementEvent;
use super::{AnimationEvent, AnimationState, Sound};

struct StackWithAnimation<'a> {
    stack: &'a Stack,
    animation: &'a mut AnimationState,
}

impl Debug for StackWithAnimation<'_> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<'a> StackWithAnimation<'a> {
    fn create_many<const N: usize>(
        handles: [StackHandle; N],
        state: &'a BattleState,
        animations: &'a mut Animations,
    ) -> [Self; N] {
        let stacks = handles.map(|h| state.get_stack(h));
        let animations = animations.0.get_many_mut(handles).unwrap();

        Iterator::zip(stacks.into_iter(), animations)
            .map(|(stack, animation)| Self { stack, animation })
            .collect::<Vec<Self>>()
            .try_into()
            .unwrap()
    }
}

pub fn animate_attack(
    attack: Attack,
    state: &BattleState,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    let [mut attacker, mut defender] =
        StackWithAnimation::create_many([attack.attacker, attack.defender], state, animations);

    let needs_turning = needs_turning(attacker.stack, defender.stack);

    equalize([attacker.animation, defender.animation]);

    if needs_turning {
        animate_turning(&mut attacker, rr);
        animate_turning(&mut defender, rr);
        equalize([attacker.animation, defender.animation]);
    }

    for strike in attack.strikes {
        let (attacker, defender) = if strike.retaliation {
            (&mut defender, &mut attacker)
        } else {
            (&mut attacker, &mut defender)
        };

        animate_strike(attacker, defender, strike.lethal, rr);
        equalize([attacker.animation, defender.animation]);
    }

    if needs_turning {
        if attacker.stack.is_alive() {
            animate_turning(&mut attacker, rr);
        }
        if defender.stack.is_alive() {
            animate_turning(&mut defender, rr);
        }
    }
}

pub fn animate_shot(
    shot: Shot,
    state: &BattleState,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    let [attacker, mut target] =
        StackWithAnimation::create_many([shot.attacker, shot.target], state, animations);

    equalize([attacker.animation, target.animation]);

    let animation_type = AnimationType::ShootStraight;
    let animation = Animation::new(animation_type, attacker.stack.creature, rr);
    let duration = animation.progress().time_left();

    put_animation_with_sound(
        attacker.animation,
        animation_type,
        attacker.stack.creature,
        rr,
    );

    target.animation.put_delay(duration);
    animate_get_hit(&mut target, shot.lethal, rr);
}

pub fn animate_movement(
    movement: Movement,
    state: &BattleState,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    let stack = state.get_stack(movement.stack_handle);
    let creature = stack.creature;
    let path = movement.path;

    let mut events = vec![];

    events.push(AnimationEvent::PlaySound(Sound {
        type_: CreatureSound::StartMoving,
        looping: false,
    }));

    let animation_type = AnimationType::StartMoving;
    if rr
        .get_creature_spritesheet(creature)
        .has_animation(animation_type)
    {
        events.push(AnimationEvent::Animation(Animation::new(
            animation_type,
            creature,
            rr,
        )));
    }

    if creature.is_teleporting() {
        events.push(AnimationEvent::Teleport(*path.last().unwrap()));
    } else {
        events.extend([
            AnimationEvent::PlaySound(Sound {
                type_: CreatureSound::Move,
                looping: true,
            }),
            AnimationEvent::Movement(MovementEvent::new(creature, path, rr)),
            AnimationEvent::StopSound,
        ]);
    }

    events.push(AnimationEvent::PlaySound(Sound {
        type_: CreatureSound::EndMoving,
        looping: false,
    }));

    let animation_type = AnimationType::StopMoving;
    if rr
        .get_creature_spritesheet(creature)
        .has_animation(animation_type)
    {
        events.push(AnimationEvent::Animation(Animation::new(
            animation_type,
            creature,
            rr,
        )));
    }

    animations
        .0
         .0
        .get_mut(&movement.stack_handle)
        .unwrap()
        .event_queue
        .extend(events);
}

fn equalize<const N: usize>(animation_states: [&mut AnimationState; N]) {
    let max_duration = animation_states
        .iter()
        .map(|state| state.total_duration())
        .max()
        .unwrap_or(Duration::ZERO);

    for state in animation_states {
        state.put_delay(max_duration - state.total_duration());
    }
}

fn animate_strike(
    attacker: &mut StackWithAnimation,
    defender: &mut StackWithAnimation,
    lethal: bool,
    rr: &mut ResourceRegistry,
) {
    let animation_type = AnimationType::AttackStraight;
    let animation = Animation::new(animation_type, attacker.stack.creature, rr);
    let animation_duration = animation.progress().time_left();

    put_animation_with_sound(
        attacker.animation,
        animation_type,
        attacker.stack.creature,
        rr,
    );

    defender.animation.put_delay(animation_duration / 2);
    animate_get_hit(defender, lethal, rr);
}

fn animate_get_hit(victim: &mut StackWithAnimation, lethal: bool, rr: &mut ResourceRegistry) {
    let animation_type = if lethal {
        AnimationType::Death
    } else if victim.stack.defending {
        AnimationType::Defend
    } else {
        AnimationType::GettingHit
    };

    put_animation_with_sound(victim.animation, animation_type, victim.stack.creature, rr);
}

fn animate_turning(stack: &mut StackWithAnimation, rr: &mut ResourceRegistry) {
    put_animation_with_sound(
        stack.animation,
        AnimationType::TurnLeft,
        stack.stack.creature,
        rr,
    );
    stack.animation.put_event(AnimationEvent::InvertSide);
    put_animation_with_sound(
        stack.animation,
        AnimationType::TurnRight,
        stack.stack.creature,
        rr,
    );
}

fn facing_side(pos: GridPos, target: GridPos) -> Side {
    assert!(pos != target);

    if pos.y == target.y {
        if pos.x > target.x {
            Side::Defender
        } else {
            Side::Attacker
        }
    } else if pos.is_even_row() {
        if pos.x <= target.x {
            Side::Attacker
        } else {
            Side::Defender
        }
    } else if pos.x >= target.x {
        Side::Defender
    } else {
        Side::Attacker
    }
}

fn needs_turning(attacker: &Stack, defender: &Stack) -> bool {
    facing_side(attacker.head, defender.head) != attacker.side
}

fn put_animation_with_sound(
    state: &mut AnimationState,
    animation_type: AnimationType,
    creature: Creature,
    rr: &mut ResourceRegistry,
) {
    if let Some(sound) = sound_for_animation(animation_type) {
        state.put_event(AnimationEvent::PlaySound(sound));
    }

    let animation = Animation::new(animation_type, creature, rr);
    state.put_event(AnimationEvent::Animation(animation));
}

fn sound_for_animation(animation_type: AnimationType) -> Option<Sound> {
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

    sound_type.map(|type_| Sound {
        type_,
        looping: false,
    })
}
