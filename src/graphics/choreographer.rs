use std::time::Duration;

use super::creature::AnimationType;
use super::animations::CreatureAnimation;
use crate::gamestate::{BattleState, GridPos};
use crate::resources::ResourceRegistry;


pub fn animate_unit_move(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, path: Vec<GridPos>) {
    let unit = state.get_unit_mut(unit_index);
    let creature = unit.creature();

    if rr.get_creature_container(creature).has_animation_block(AnimationType::StartMoving) {
        unit.push_animation(CreatureAnimation::new(AnimationType::StartMoving));
    }

    for _ in path {
        unit.push_animation(CreatureAnimation::new(AnimationType::Moving));
    }

    if rr.get_creature_container(creature).has_animation_block(AnimationType::StopMoving) {
        unit.push_animation(CreatureAnimation::new(AnimationType::StopMoving));
    }

    unit.push_animation(CreatureAnimation::new_looping(AnimationType::Standing));
}
