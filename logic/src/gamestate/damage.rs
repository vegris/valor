use gamedata::heroes::{Ability, AbilityLevel};
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use super::hero::Hero;
use crate::stack::Stack;

pub enum AttackType {
    Melee,
    Shoot,
}

pub fn deal_damage(
    heroes: &[Option<Hero>; 2],
    attacker: &Stack,
    defender: &mut Stack,
    attack_type: AttackType,
) {
    let damage = calculate_damage(heroes, attacker, defender, attack_type);
    defender.receive_damage(damage);
}

fn calculate_damage(
    heroes: &[Option<Hero>; 2],
    attacker: &Stack,
    defender: &Stack,
    attack_type: AttackType,
) -> i32 {
    let attacker_hero = heroes[attacker.side as usize].as_ref();
    let defender_hero = heroes[defender.side as usize].as_ref();

    let base = base_damage(attacker) as f32;

    let (md1, md2) = primary_damage_modifiers(attacker_hero, attacker, defender_hero, defender);

    let offence_md = offence_modifier(attacker_hero, attack_type);
    let armor_md = armorer_modifier(defender_hero);

    let damage = base * (1.0 + md1 + offence_md) * md2 * armor_md;

    damage.floor() as i32
}

fn base_damage(stack: &Stack) -> i32 {
    let (damage_low, damage_high) = stack.creature.base_stats().damage;

    if damage_low == damage_high {
        return damage_low * stack.count;
    }

    let sum: i32 = Uniform::from(damage_low..damage_high)
        .sample_iter(rand::thread_rng())
        .take(i32::min(stack.count, 10) as usize)
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
    let attack = attacker.base_stats().attack + attacker_hero.map_or(0, |h| h.stats.attack);
    let attack = attack as f32;

    let defence = defender.base_stats().defence + defender_hero.map_or(0, |h| h.stats.defence);
    let defence = defence as f32;

    // TODO: bring back ignore defence
    // let defence = if let Some(reduction_percent) = attacker.creature.ignore_defence() {
    //     (defence * (1.0 - reduction_percent)).ceil() - 1.0
    // } else {
    //     defence
    // };

    let md1 = if attack > defence {
        0.05 * (attack - defence)
    } else {
        0.0
    };
    let md1 = f32::min(md1, 3.0);

    let md2 = if defence > attack {
        1.0 - 0.025 * (defence - attack)
    } else {
        1.0
    };
    let md2 = f32::max(md2, 0.3);

    (md1, md2)
}

fn offence_modifier(attacker_hero: Option<&Hero>, attack_type: AttackType) -> f32 {
    fn offence(level: AbilityLevel) -> f32 {
        match level {
            AbilityLevel::Basic => 0.1,
            AbilityLevel::Advanced => 0.2,
            AbilityLevel::Expert => 0.3,
        }
    }

    fn archery(level: AbilityLevel) -> f32 {
        match level {
            AbilityLevel::Basic => 0.1,
            AbilityLevel::Advanced => 0.25,
            AbilityLevel::Expert => 0.5,
        }
    }

    type ModifierFN = fn(AbilityLevel) -> f32;
    let (ability, modifier_fun): (Ability, ModifierFN) = match attack_type {
        AttackType::Melee => (Ability::Offense, offence),
        AttackType::Shoot => (Ability::Archery, archery),
    };

    attacker_hero
        .and_then(|h| h.get_ability_level(ability))
        .map_or(0.0, modifier_fun)
}

fn armorer_modifier(defender_hero: Option<&Hero>) -> f32 {
    let m = defender_hero
        .and_then(|h| h.get_ability_level(Ability::Armorer))
        .map_or(0.0, |l| match l {
            AbilityLevel::Basic => 0.05,
            AbilityLevel::Advanced => 0.1,
            AbilityLevel::Expert => 0.15,
        });

    1.0 - m
}
