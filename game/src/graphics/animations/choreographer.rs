use std::fmt::Debug;
use std::time::Duration;

use crate::battlestate::{BattleState, Side, StackHandle};
use crate::event::{Attack, Movement, Shot};
use crate::graphics::spritesheet::creature::AnimationType;
use crate::graphics::Animations;
use crate::grid::GridPos;
use crate::registry::ResourceRegistry;
use crate::stack::Stack;

use super::animation::Animation;
use super::{AnimationEvent, AnimationState};

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

        Iterator::zip(stacks.into_iter(), animations.into_iter())
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
    let duration = animation.progress.time_left();

    attacker
        .animation
        .put_animation(animation_type, attacker.stack.creature, rr);

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
    let stack_animations = animations.0 .0.get_mut(&movement.stack_handle).unwrap();
    stack_animations.put_movement(stack.creature, movement.path, rr);
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
    let animation_duration = animation.progress.time_left();

    attacker
        .animation
        .put_animation(animation_type, attacker.stack.creature, rr);
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
    victim
        .animation
        .put_animation(animation_type, victim.stack.creature, rr);
}

fn animate_turning(stack: &mut StackWithAnimation, rr: &mut ResourceRegistry) {
    stack
        .animation
        .put_animation(AnimationType::TurnLeft, stack.stack.creature, rr);
    stack.animation.put_event(AnimationEvent::InvertSide);
    stack
        .animation
        .put_animation(AnimationType::TurnRight, stack.stack.creature, rr);
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
