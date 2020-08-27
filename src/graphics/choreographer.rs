use std::time::Instant;

use super::creature::AnimationType;
use super::animations::{Tweening, Animation};
use crate::gamestate::{BattleState, GridPos};
use crate::resources::ResourceRegistry;


pub fn animate_move_unit(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, path: Vec<GridPos>, start_from: Instant) {
    let unit = state.get_unit_mut(unit_index);

    let animation = Animation::new(AnimationType::StartMoving, start_from);
    let mut animation_end = animation.end();
    unit.push_animation(animation);

    let mut current_pos = unit.current_pos();
    for next_grid in path {
        let next_pos = next_grid.draw_pos();
        
        let tweening = Tweening::new(current_pos, next_pos, animation_end);
        unit.push_tweening(tweening);
        current_pos = next_pos;

        let move_animation = Animation::new(AnimationType::Moving, animation_end);
        animation_end = move_animation.end();
        unit.push_animation(move_animation);
    }

    if rr.get_creature_container(unit.creature()).get_animation_block(AnimationType::StopMoving).is_some() {
        let animation = Animation::new(AnimationType::StopMoving, animation_end);
        animation_end = animation.end();
        unit.push_animation(animation);
    }
    let animation = Animation::new_looping(AnimationType::Standing, animation_end, None);
    unit.push_animation(animation);
}