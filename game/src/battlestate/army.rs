use gamedata::Creature;

use crate::grid::GridPos;
use crate::pathfinding::head_from_tail;
use crate::registry::ResourceRegistry;
use crate::stack::Stack;

use super::Side;

pub fn initial_placement(units_count: u8) -> Vec<i32> {
    match units_count {
        1 => vec![6],
        2 => vec![3, 9],
        3 => vec![3, 6, 9],
        4 => vec![1, 5, 7, 11],
        5 => vec![1, 3, 6, 9, 11],
        6 => vec![1, 3, 5, 7, 9, 11],
        7 => vec![1, 3, 5, 6, 7, 9, 11],
        _ => unreachable!(),
    }
}

pub fn form_units(
    starting_army: &[Option<(Creature, i32)>; 7],
    side: Side,
    rr: &mut ResourceRegistry,
) -> Vec<Stack> {
    let units_count = starting_army.iter().filter(|c| c.is_some()).count();
    let formation = initial_placement(units_count as u8);
    let starting_x = *match side {
        Side::Attacker => GridPos::X_RANGE.start(),
        Side::Defender => GridPos::X_RANGE.end(),
    };
    starting_army
        .iter()
        .filter_map(|c| *c)
        .zip(formation.into_iter())
        .map(|((creature, count), y_pos)| {
            let head = head_from_tail(creature, side, GridPos::new(starting_x, y_pos));
            Stack::new(creature, count, head, side, rr)
        })
        .collect()
}
