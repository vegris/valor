use super::creature::AnimationType;
use super::animations::CreatureAnimation;
use crate::gamestate::{BattleState, GridPos};
use crate::resources::ResourceRegistry;
use crate::gamestate::creature::Direction;


pub fn animate_unit_move(state: &mut BattleState, rr: &mut ResourceRegistry, unit_index: usize, path: &Vec<GridPos>) {
    let unit = state.get_unit_mut(unit_index);
    let creature = unit.creature();

    if rr.get_creature_container(creature).has_animation_block(AnimationType::StartMoving) {
        unit.push_animation(CreatureAnimation::new(AnimationType::StartMoving));
    }

    let mut cur_grid_pos = unit.grid_pos();
    let mut cur_direction = unit.direction;
    for grid_pos in &path[1..] {
        if need_turning(cur_grid_pos, *grid_pos, cur_direction) {
            unit.push_animation(CreatureAnimation::new_turning(AnimationType::TurnLeft));
            unit.push_animation(CreatureAnimation::new(AnimationType::TurnRight));
            cur_direction = cur_direction.inversion();
        }
        unit.push_animation(CreatureAnimation::new_tweening(cur_grid_pos.draw_pos(), grid_pos.draw_pos()));
        cur_grid_pos = *grid_pos;
    }

    if rr.get_creature_container(creature).has_animation_block(AnimationType::StopMoving) {
        unit.push_animation(CreatureAnimation::new(AnimationType::StopMoving));
    }

    unit.push_animation(CreatureAnimation::new_looping(AnimationType::Standing));
}

fn need_turning(current_pos: GridPos, next_pos: GridPos, current_direction: Direction) -> bool {
    let needed_direction = 
        if current_pos.is_even_row() {
            if next_pos.x() < current_pos.x() {
                Direction::Left
            } else {
                Direction::Right
            }
        } else {
            if next_pos.x() > current_pos.x() {
                Direction::Right
            } else {
                Direction::Left
            }
        };

    current_direction != needed_direction
}
