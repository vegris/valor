use crate::battlestate::turns;
use crate::battlestate::BattleState;
use crate::command::Wait;

use super::Event;

pub fn is_applicable(_command: Wait, state: &BattleState) -> bool {
    state
        .get_current_stack()
        .turn_state
        .map_or(false, |phase| phase == turns::Phase::Fresh)
}
pub fn apply(_command: Wait, state: &mut BattleState) -> Vec<Event> {
    let current_stack = state.get_current_stack_mut();
    current_stack.turn_state = Some(turns::Phase::Wait);

    vec![]
}
