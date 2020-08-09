use std::time::Instant;

use crate::time_progress::{Tweening, Animation};
use crate::battle::{BattleState, GridPos};
use crate::resources::AnimationType;

pub fn animate_move_unit(state: &mut BattleState, unit_index: usize, path: Vec<GridPos>, start_from: Instant) {
    let unit = state.get_unit_mut(unit_index);
    let mut current_pos = unit.current_pos();
    let mut next_start_from = start_from;

    for next_grid in path {
        let next_pos = next_grid.draw_pos();
        
        let tweening = Tweening::new(current_pos, next_pos, next_start_from);
        let ends_at = tweening.end();
        unit.push_tweening(tweening);

        current_pos = next_pos;
        next_start_from = ends_at;
    }
    let move_animation = Animation::new_looping(AnimationType::Moving, start_from, Some(next_start_from));
    unit.push_animation(move_animation);
    let end_move_animaton = Animation::new_looping(AnimationType::StopMoving, next_start_from, None);
    let end_move_animation_end = end_move_animaton.end();
    unit.push_animation(end_move_animaton);
    let standing_animation = Animation::new_looping(AnimationType::StopMoving, end_move_animation_end, None);
    unit.push_animation(standing_animation);
}