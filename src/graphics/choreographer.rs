use std::time::Instant;

use super::creature::AnimationType;
use super::animations::CreatureAnimation;
use crate::gamestate::{BattleState, GridPos};
use crate::resources::ResourceRegistry;


pub fn animate_unit_move(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, path: Vec<GridPos>, start_from: Instant) {
    let unit = state.get_unit_mut(unit_index);

    let mut animation_end = start_from;

    if rr.get_creature_container(unit.creature()).get_animation_block(AnimationType::StartMoving).is_some() {
        let animation = CreatureAnimation::new_ordinary(AnimationType::StartMoving, animation_end);
        animation_end = animation.end();
        unit.push_animation(animation);
    }

    let mut current_pos = unit.current_pos();
    for next_grid in path {
        let next_pos = next_grid.draw_pos();

        let move_animation = CreatureAnimation::new_tweening(animation_end, current_pos, next_pos);

        current_pos = next_pos;
        animation_end = move_animation.end();

        unit.push_animation(move_animation);
    }

    if rr.get_creature_container(unit.creature()).get_animation_block(AnimationType::StopMoving).is_some() {
        let animation = CreatureAnimation::new_ordinary(AnimationType::StopMoving, animation_end);
        animation_end = animation.end();
        unit.push_animation(animation);
    }


    // animate_unit_turning(state, rr, unit_index, animation_end);
    // animate_unit_standing(state, rr, unit_index, unit.animation_queue.back().unwrap().end());
}

// pub fn animate_unit_standing(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, start_from: Instant) {
//     let unit = state.get_unit_mut(unit_index);
//     let animation = Animation::new_looping(AnimationType::Standing, start_from);
//     unit.push_animation(animation);
// }

// pub fn animate_unit_turning(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, start_from: Instant) {
//     let unit = state.get_unit_mut(unit_index);
//     let animation_type = 
//         if unit.face_left {
//             AnimationType::TurnRight
//         } else {
//             AnimationType::TurnLeft
//         };
//     let animation = Animation::new(animation_type, start_from);
//     unit.push_animation(animation);
// }