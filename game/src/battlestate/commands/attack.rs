use gamedata::Ability;

use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::{battlestate::BattleState, pathfinding, stack::Stack};

use super::{r#move, CommandT};

impl CommandT for crate::command::Attack {
    fn is_applicable(&self, state: &BattleState) -> bool {
        let current_stack = state.get_current_stack();
        let current_side = current_stack.side;
        let is_wide = current_stack.creature.is_wide();

        let occupied_cells = pathfinding::get_occupied_cells_for(current_stack.creature, current_side, current_stack.head).unwrap();

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
            .filter(|creature_pos| {
                occupied_cells.contains(creature_pos) || r#move::is_applicable(state, *creature_pos)}
            )
            .is_some()
    }

    fn apply(self, state: &mut BattleState) {
        let current_stack = state.get_current_stack();
        let is_wide = current_stack.creature.is_wide();

        let initial_position = current_stack.head;

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

        let damage = calculate_damage(attacker, defender);
        defender.receive_damage(damage);

        if defender.is_alive()
            && defender.retaliation_count.has_retaliation()
            && !attacker.creature.has_ability(Ability::NoRetaliation)
        {
            defender.retaliation_count.decrement();
            let damage = calculate_damage(defender, attacker);
            attacker.receive_damage(damage);
        }

        if defender.is_alive()
            && attacker.is_alive()
            && attacker.creature.has_ability(Ability::DoubleStrike)
        {
            println!("Using double strike!");
            let damage = calculate_damage(attacker, defender);
            defender.receive_damage(damage);
        }

        if attacker.is_alive() && attacker.creature.has_ability(Ability::ReturnAfterStrike) {
            r#move::apply(state, initial_position);
        }
    }
}

fn calculate_damage(attacker: &Stack, defender: &Stack) -> i32 {
    base_damage(attacker)
}

fn base_damage(stack: &Stack) -> i32 {
    let random_count = i32::min(stack.count, 10) as usize;

    let (damage_low, damage_high) = stack.creature.base_stats().damage;
    let between = Uniform::try_from(damage_low..damage_high).unwrap();

    let sum: i32 = between
        .sample_iter(rand::thread_rng())
        .take(random_count)
        .sum();

    if stack.count <= 10 {
        sum
    } else {
        (0.1 * stack.count as f32 * sum as f32).ceil() as i32
    }
}
