use gamedata::Ability;

use crate::battlestate::damage;
use crate::battlestate::BattleState;

use super::CommandT;

const ATTACK_TYPE: damage::AttackType = damage::AttackType::Shoot;

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
        let [attacker, defender] = state
            .stacks
            .get_many_mut([state.current_stack, self.target])
            .unwrap();

        attacker.current_ammo -= 1;

        damage::deal_damage(&state.heroes, attacker, defender, ATTACK_TYPE);

        if !defender.is_alive() {
            return;
        }

        if attacker.current_ammo > 0 && attacker.creature.has_ability(Ability::DoubleShot) {
            println!("Using double shot!");

            attacker.current_ammo -= 1;
            damage::deal_damage(&state.heroes, attacker, defender, ATTACK_TYPE);
        }
    }
}
