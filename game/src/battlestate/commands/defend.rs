use crate::{battlestate::BattleState, command::Defend};

use super::Event;

pub fn is_applicable(_command: Defend, _state: &BattleState) -> bool {
    true
}
pub fn apply(_command: Defend, state: &mut BattleState) -> Vec<Event> {
    let current_stack = state.get_current_stack_mut();
    current_stack.defending = true;

    vec![]
}
