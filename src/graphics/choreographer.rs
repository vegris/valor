use std::time::Duration;

use super::creature::AnimationType;
use super::animations::CreatureAnimation;
use crate::gamestate::{BattleState, GridPos};
use crate::resources::ResourceRegistry;


pub fn animate_unit_move(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, path: Vec<GridPos>) {
    let unit = state.get_unit_mut(unit_index);

    unit.push_animation(CreatureAnimation::new_delayed(AnimationType::Standing, Duration::from_secs(1)));

    if rr.get_creature_container(unit.creature()).has_animation_block(AnimationType::StartMoving) {
        unit.push_animation(CreatureAnimation::new(AnimationType::StartMoving));
    }

    let mut current_pos = unit.current_pos();
    for next_grid in path {
        let next_pos = next_grid.draw_pos();

        let move_animation = CreatureAnimation::new_tweening(current_pos, next_pos);
        unit.push_animation(move_animation);

        current_pos = next_pos;
    }

    if rr.get_creature_container(unit.creature()).has_animation_block(AnimationType::StopMoving) {
        unit.push_animation(CreatureAnimation::new(AnimationType::StopMoving));
    }

    unit.push_animation(CreatureAnimation::new_looping());

    unit.push_animation(CreatureAnimation::new(AnimationType::TurnLeft));
    unit.push_animation(CreatureAnimation::new_turning());
}
