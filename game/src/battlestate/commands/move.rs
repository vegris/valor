use crate::{battlestate::BattleState, grid::GridPos};

use super::Event;

impl crate::command::Move {
    pub fn is_applicable(&self, state: &BattleState) -> bool {
        is_applicable(state, self.destination)
    }
    pub fn apply(self, state: &mut BattleState) -> Vec<Event> {
        apply(state, self.destination)
    }
}

pub fn is_applicable(state: &BattleState, destination: GridPos) -> bool {
    let current_stack = state.get_current_stack();

    let is_position_available = crate::pathfinding::get_occupied_cells_for(
        current_stack.creature,
        current_stack.side,
        destination,
    )
    .map(|cells| {
        cells
            .into_iter()
            .map(|cell| state.find_unit_for_cell(cell))
            .all(|option| option.is_none())
    })
    .unwrap_or(false);

    is_position_available && state.reachable_cells.contains(&destination)
}

pub fn apply(state: &mut BattleState, destination: GridPos) -> Vec<Event> {
    let path = state
        .navigation_array
        .get_shortest_path(destination)
        .unwrap();

    let current_stack = state.get_current_stack_mut();

    current_stack.head = destination;

    let mut events = vec![];
    if path.len() > 0 {
        events.push(Event::Movement {
            stack_handle: state.current_stack,
            path,
        });
    }

    events
}
