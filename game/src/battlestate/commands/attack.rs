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

        let defender_handle = state.find_unit_for_cell(self.attack_position).unwrap();

        let [attacker, defender] = state
            .get_stacks_mut([state.current_stack, defender_handle])
            .unwrap();

        defender.count -= 1;

        if defender.is_alive() && defender.retaliation_count.has_retaliation() {
            defender.retaliation_count.decrement();
            attacker.count -= 1;
        }
    }
}
