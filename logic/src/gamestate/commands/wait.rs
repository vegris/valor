use super::Event;
use crate::gamestate::GameState;
use crate::turn::Phase;

pub fn is_applicable(state: &GameState) -> bool {
    state
        .get_current_stack()
        .turn_state
        .map_or(false, |phase| phase == Phase::Fresh)
}
pub fn apply(state: &mut GameState) -> Vec<Event> {
    let current_stack = state.get_current_stack_mut();
    current_stack.turn_state = Some(Phase::Wait);

    vec![]
}
