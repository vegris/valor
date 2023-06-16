use crate::battlestate::BattleState;

use super::CommandT;

impl CommandT for crate::command::Defend {
    fn is_applicable(&self, _state: &BattleState) -> bool {
        true
    }
    fn apply(self, state: &mut BattleState) {
        let current_stack = state.get_current_stack_mut();
        current_stack.defending = true;
    }
}
