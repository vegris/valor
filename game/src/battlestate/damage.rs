use rand::distributions::Uniform;
use rand::prelude::Distribution;

use gamedata::heroes::abilities::{Ability, Level};

use crate::stack::Stack;

use super::hero::Hero;

pub fn deal_damage(heroes: &[Option<Hero>; 2], attacker: &Stack, defender: &mut Stack) {
    let damage = calculate_damage(heroes, attacker, defender);
    defender.receive_damage(damage);
}

fn calculate_damage(heroes: &[Option<Hero>; 2], attacker: &Stack, defender: &Stack) -> i32 {
    let attacker_hero = heroes[attacker.side as usize].as_ref();
    let defender_hero = heroes[defender.side as usize].as_ref();

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
    let attack = attack as f32;

    let defence =
        defender.base_stats().defence as i32 + defender_hero.map_or(0, |h| h.stats.defence);
    let defence = defence as f32;

    let defence = if let Some(reduction_percent) = attacker.creature.ignore_defence() {
        (defence * (1.0 - reduction_percent)).ceil() - 1.0
    } else {
        defence
    };

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

fn offence_modifier(attacker_hero: Option<&Hero>) -> f32 {
    attacker_hero
        .and_then(|h| h.get_ability_level(Ability::Offense))
        .map_or(0.0, |l| match l {
            Level::Basic => 0.1,
            Level::Advanced => 0.2,
            Level::Expert => 0.3,
        })
}

fn armorer_modifier(defender_hero: Option<&Hero>) -> f32 {
    let m = defender_hero
        .and_then(|h| h.get_ability_level(Ability::Armorer))
        .map_or(0.0, |l| match l {
            Level::Basic => 0.05,
            Level::Advanced => 0.1,
            Level::Expert => 0.15,
        });

    1.0 - m
}
