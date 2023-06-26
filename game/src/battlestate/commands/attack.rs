use gamedata::heroes::abilities as hero_abilities;
use gamedata::Ability;

use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::{
    battlestate::{hero::Hero, BattleState},
    pathfinding,
    stack::Stack,
};

use super::{r#move, CommandT};

impl CommandT for crate::command::Attack {
    fn is_applicable(&self, state: &BattleState) -> bool {
        let current_stack = state.get_current_stack();
        let current_side = current_stack.side;
        let is_wide = current_stack.creature.is_wide();

        let occupied_cells = pathfinding::get_occupied_cells_for(
            current_stack.creature,
            current_side,
            current_stack.head,
        )
        .unwrap();

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
                occupied_cells.contains(creature_pos) || r#move::is_applicable(state, *creature_pos)
            })
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

        let mut attacker = state.stacks.remove(&state.current_stack).unwrap();
        let mut defender = state.stacks.remove(&defender_handle).unwrap();

        deal_damage(state, &attacker, &mut defender);

        if defender.is_alive()
            && defender.retaliation_count.has_retaliation()
            && !attacker.creature.has_ability(Ability::NoRetaliation)
        {
            defender.retaliation_count.decrement();
            deal_damage(state, &defender, &mut attacker);
        }

        if defender.is_alive()
            && attacker.is_alive()
            && attacker.creature.has_ability(Ability::DoubleStrike)
        {
            println!("Using double strike!");
            deal_damage(state, &attacker, &mut defender);
        }

        if attacker.is_alive() && attacker.creature.has_ability(Ability::ReturnAfterStrike) {
            r#move::apply(state, initial_position);
        }

        state.stacks.insert(state.current_stack, attacker);
        state.stacks.insert(defender_handle, defender);
    }
}

fn deal_damage(state: &mut BattleState, attacker: &Stack, defender: &mut Stack) {
    let damage = calculate_damage(state, attacker, defender);
    defender.receive_damage(damage);
}

fn calculate_damage(state: &BattleState, attacker: &Stack, defender: &Stack) -> i32 {
    let attacker_hero = state.heroes[attacker.side as usize].as_ref();
    let defender_hero = state.heroes[defender.side as usize].as_ref();

    let base = base_damage(attacker) as f32;

    let (md1, md2) = primary_damage_modifiers(attacker_hero, attacker, defender_hero, defender);

    let offence_md = offence_modifier(attacker_hero);
    let armor_md = armorer_modifier(defender_hero);

    let damage = base * (1.0 + md1 + offence_md) * md2 * armor_md;

    damage.floor() as i32
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

fn primary_damage_modifiers(
    attacker_hero: Option<&Hero>,
    attacker: &Stack,
    defender_hero: Option<&Hero>,
    defender: &Stack,
) -> (f32, f32) {
    let attack = attacker.base_stats().attack as i32 + attacker_hero.map_or(0, |h| h.stats.attack);
    let defence =
        defender.base_stats().defence as i32 + defender_hero.map_or(0, |h| h.stats.defence);

    let md1 = if attack > defence {
        0.05 * (attack - defence) as f32
    } else {
        0.0
    };
    let md1 = f32::min(md1, 3.0);

    let md2 = if defence > attack {
        1.0 - 0.025 * (defence - attack) as f32
    } else {
        1.0
    };
    let md2 = f32::max(md2, 0.3);

    (md1, md2)
}

fn offence_modifier(attacker_hero: Option<&Hero>) -> f32 {
    attacker_hero
        .and_then(|h| h.get_ability_level(hero_abilities::Ability::Offense))
        .map_or(0.0, |l| match l {
            hero_abilities::Level::Basic => 0.1,
            hero_abilities::Level::Advanced => 0.2,
            hero_abilities::Level::Expert => 0.3,
        })
}

fn armorer_modifier(defender_hero: Option<&Hero>) -> f32 {
    let m = defender_hero
        .and_then(|h| h.get_ability_level(hero_abilities::Ability::Armorer))
        .map_or(0.0, |l| match l {
            hero_abilities::Level::Basic => 0.05,
            hero_abilities::Level::Advanced => 0.1,
            hero_abilities::Level::Expert => 0.15,
        });

    1.0 - m
}
