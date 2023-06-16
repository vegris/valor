use gamedata::Ability;

use crate::battlestate::BattleState;

use super::CommandT;

impl CommandT for crate::command::Shoot {
    fn is_applicable(&self, state: &BattleState) -> bool {
        let current_stack = state.get_current_stack();
        let target_stack = state.get_stack(self.target);

        let is_enemy = current_stack.side != target_stack.side;
        let is_alive = state.get_stack(self.target).is_alive();
        let can_shoot = current_stack.can_shoot(state);

        is_enemy && is_alive && can_shoot
    }
    fn apply(self, state: &mut BattleState) {
        let mut attack_stack = state.get_current_stack_mut();
        attack_stack.current_ammo -= 1;

        let mut defend_stack = state.get_stack_mut(self.target);
        defend_stack.count -= 1;

        if !defend_stack.is_alive() {
            return;
        }

        let attack_stack = state.get_current_stack();
        if attack_stack.creature.has_ability(Ability::DoubleShot) && attack_stack.can_shoot(state) {
            println!("Using double shot!");

            let mut attack_stack = state.get_current_stack_mut();
            attack_stack.current_ammo -= 1;

            let mut defend_stack = state.get_stack_mut(self.target);
            defend_stack.count -= 1;
        }
    }
}
