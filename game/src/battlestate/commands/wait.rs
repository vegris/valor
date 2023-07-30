use crate::battlestate::turns;
use crate::battlestate::BattleState;

use super::Event;

pub fn is_applicable(state: &BattleState) -> bool {
    state
        .get_current_stack()
        .turn_state
        .map_or(false, |phase| phase == turns::Phase::Fresh)
}
pub fn apply(state: &mut BattleState) -> Vec<Event> {
    let current_stack = state.get_current_stack_mut();
    current_stack.turn_state = Some(turns::Phase::Wait);

    vec![]
}
