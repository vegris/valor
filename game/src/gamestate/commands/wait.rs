use crate::gamestate::turns;
use crate::gamestate::GameState;

use super::Event;

pub fn is_applicable(state: &GameState) -> bool {
    state
        .get_current_stack()
        .turn_state
        .map_or(false, |phase| phase == turns::Phase::Fresh)
}
pub fn apply(state: &mut GameState) -> Vec<Event> {
    let current_stack = state.get_current_stack_mut();
    current_stack.turn_state = Some(turns::Phase::Wait);

    vec![]
}
