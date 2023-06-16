use crate::battlestate::turns;
use crate::battlestate::BattleState;

use super::CommandT;

impl CommandT for crate::command::Wait {
    fn is_applicable(self, state: &BattleState) -> bool {
        state
            .get_current_stack()
            .turn_state
            .map_or(false, |phase| phase == turns::Phase::Fresh)
    }
    fn apply(self, state: &mut BattleState) {
        let current_stack = state.get_current_stack_mut();
        current_stack.turn_state = Some(turns::Phase::Wait);
    }
}
