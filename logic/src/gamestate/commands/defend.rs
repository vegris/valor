use super::Event;
use crate::gamestate::GameState;

pub fn is_applicable(_state: &GameState) -> bool {
    true
}
pub fn apply(state: &mut GameState) -> Vec<Event> {
    let current_stack = state.get_current_stack_mut();
    current_stack.defending = true;

    vec![]
}
