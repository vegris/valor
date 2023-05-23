use gridpos::GridPos;

use crate::{
    animations::Animation, creature_stack::CreatureStack, graphics::creature::AnimationType,
    pathfinding,
};

pub fn animate_moving(creature_stack: &mut CreatureStack, path: Vec<GridPos>) {
    let CreatureStack {
        creature,
        side,
        ref mut graphics,
        ..
    } = creature_stack;

    graphics.animation_queue.add(Animation::new(AnimationType::StartMoving));

    let creature_positions = path
        .into_iter()
        .map(|gridpos| pathfinding::tail_for(*creature, *side, gridpos).unwrap());

    let iterator = Iterator::zip(creature_positions.clone(), creature_positions.skip(1));
    for (from, to) in iterator {
        let animation = Animation::new_with_tween(AnimationType::Moving, from, to);
        graphics.animation_queue.add(animation);
    }

    graphics.animation_queue.add(Animation::new(AnimationType::StopMoving));
}
