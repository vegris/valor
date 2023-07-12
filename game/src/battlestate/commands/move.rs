use crate::{battlestate::BattleState, command::Move};

use super::Event;

pub fn is_applicable(command: Move, state: &BattleState) -> bool {
    let current_stack = state.get_current_stack();

    let is_position_available = crate::pathfinding::get_occupied_cells_for(
        current_stack.creature,
        current_stack.side,
        command.destination,
    )
    .map(|cells| {
        cells
            .into_iter()
            .map(|cell| state.find_unit_for_cell(cell))
            .all(|option| option.is_none())
    })
    .unwrap_or(false);

    is_position_available && state.reachable_cells.contains(&command.destination)
}

pub fn apply(command: Move, state: &mut BattleState) -> Vec<Event> {
    let path = state
        .navigation_array
        .get_shortest_path(command.destination)
        .unwrap();

    let current_stack = state.get_current_stack_mut();

    current_stack.head = command.destination;

    let mut events = vec![];
    if !path.is_empty() {
        events.push(Event::Movement {
            stack_handle: state.current_stack,
            path,
        });
    }

    events
}
