use crate::{battlestate::BattleState, pathfinding};

use super::{r#move, CommandT};

impl CommandT for crate::command::Attack {
    fn is_applicable(&self, state: &BattleState) -> bool {
        let current_stack = state.get_current_stack();
        let current_side = current_stack.side;
        let is_wide = current_stack.creature.is_wide();

        let potential_pos = pathfinding::unit_position_for_attack(
            self.attack_position,
            self.attack_direction,
            current_side,
            is_wide,
        );

        // успех в случае
        // 1. на позиции есть существо
        // 2. оно - враг
        // 3. атакующий может дойти до позиции атаки и поместиться там
        state
            .find_unit_for_cell(self.attack_position)
            .map(|handle| state.get_stack(handle))
            .filter(|stack| stack.side != current_side)
            .and(potential_pos)
            .filter(|&creature_pos| r#move::is_applicable(state, creature_pos))
            .is_some()
    }

    fn apply(self, state: &mut BattleState) {
        let current_stack = state.get_current_stack();
        let is_wide = current_stack.creature.is_wide();

        let position = pathfinding::unit_position_for_attack(
            self.attack_position,
            self.attack_direction,
            current_stack.side,
            is_wide,
        )
        .unwrap();

        r#move::apply(state, position);

        let _current_stack = state.get_current_stack_mut();

        let defending_unit_handle = state.find_unit_for_cell(self.attack_position).unwrap();
        let mut defending_unit = state.get_stack_mut(defending_unit_handle);
        defending_unit.count -= 1;

        if defending_unit.is_alive() && !defending_unit.counterattacked {
            defending_unit.counterattacked = true;
            let mut current_stack = state.get_current_stack_mut();
            current_stack.count -= 1;
        }
    }
}
