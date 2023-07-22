use std::time::Duration;

use crate::battlestate::{BattleState, Side};
use crate::event::{Attack, Movement, Shot};
use crate::graphics::spritesheet::creature::AnimationType;
use crate::graphics::Animations;
use crate::grid::GridPos;
use crate::registry::ResourceRegistry;
use crate::stack::Stack;

use super::animation::Animation;
use super::{AnimationEvent, AnimationState};

pub fn animate_attack(
    attack: Attack,
    state: &BattleState,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    let attacker_stack = state.get_stack(attack.attacker);
    let defender_stack = state.get_stack(attack.defender);

    let [attacker, defender] = animations
        .get_many_mut([attack.attacker, attack.defender])
        .unwrap();

    let needs_turning = needs_turning(attacker_stack, defender_stack);

    equalize([attacker, defender]);

    if needs_turning {
        attacker.put_animation(AnimationType::TurnLeft, attacker_stack.creature, rr);
        attacker.put_event(AnimationEvent::InvertSide);
        attacker.put_animation(AnimationType::TurnRight, attacker_stack.creature, rr);

        defender.put_animation(AnimationType::TurnLeft, defender_stack.creature, rr);
        defender.put_event(AnimationEvent::InvertSide);
        defender.put_animation(AnimationType::TurnRight, defender_stack.creature, rr);

        equalize([attacker, defender]);
    }

    for strike in attack.strikes {
        if strike.retaliation {
            animate_strike(
                defender,
                attacker,
                defender_stack,
                attacker_stack,
                strike.lethal,
                rr,
            );
        } else {
            animate_strike(
                attacker,
                defender,
                attacker_stack,
                defender_stack,
                strike.lethal,
                rr,
            );
        }

        equalize([attacker, defender]);
    }

    if needs_turning {
        if attacker_stack.is_alive() {
            attacker.put_animation(AnimationType::TurnLeft, attacker_stack.creature, rr);
            attacker.put_event(AnimationEvent::InvertSide);
            attacker.put_animation(AnimationType::TurnRight, attacker_stack.creature, rr);
        }

        if defender_stack.is_alive() {
            defender.put_animation(AnimationType::TurnLeft, defender_stack.creature, rr);
            defender.put_event(AnimationEvent::InvertSide);
            defender.put_animation(AnimationType::TurnRight, defender_stack.creature, rr);
        }
    }
}

pub fn animate_shot(
    shot: Shot,
    state: &BattleState,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    let attacker_stack = state.get_stack(shot.attacker);
    let defender_stack = state.get_stack(shot.target);

    let [attacker, defender] = animations
        .get_many_mut([shot.attacker, shot.target])
        .unwrap();

    equalize([attacker, defender]);

    let animation_type = AnimationType::ShootStraight;
    let animation = Animation::new(animation_type, attacker_stack.creature, rr);
    let duration = animation.progress.time_left();

    attacker.put_animation(animation_type, attacker_stack.creature, rr);

    defender.put_delay(duration);
    animate_get_hit(defender, defender_stack, shot.lethal, rr);
}

pub fn animate_movement(
    movement: Movement,
    state: &BattleState,
    animations: &mut Animations,
    rr: &mut ResourceRegistry,
) {
    let stack = state.get_stack(movement.stack_handle);
    let stack_animations = animations.0.get_mut(&movement.stack_handle).unwrap();
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
    attacker: &mut AnimationState,
    defender: &mut AnimationState,
    attacker_stack: &Stack,
    defender_stack: &Stack,
    lethal: bool,
    rr: &mut ResourceRegistry,
) {
    let animation_type = AnimationType::AttackStraight;
    let animation = Animation::new(animation_type, attacker_stack.creature, rr);
    let animation_duration = animation.progress.time_left();

    attacker.put_animation(animation_type, attacker_stack.creature, rr);
    defender.put_delay(animation_duration / 2);
    animate_get_hit(defender, defender_stack, lethal, rr);
}

fn animate_get_hit(
    animation_state: &mut AnimationState,
    stack: &Stack,
    lethal: bool,
    rr: &mut ResourceRegistry,
) {
    let animation_type = if lethal {
        AnimationType::Death
    } else if stack.defending {
        AnimationType::Defend
    } else {
        AnimationType::GettingHit
    };
    animation_state.put_animation(animation_type, stack.creature, rr);
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
