#![allow(unused)]

use std::cmp::{min, max};

mod creatures;
mod creature_stack;
mod skills;

use creatures::{Creature, CreatureAbility};
use creature_stack::CreatureStack;
use skills::{Effect, Level, AppliedEffect, HeroAbility, Artifact};

enum StrikeType {
    Melee,
    Ranged
}

struct Hero {
    attack: u8,
    defence: u8,
    spell_power: u8,
    knowledge: u8,
    
    skills: Vec<(HeroAbility, Level)>,
    artifacts: Vec<Artifact>
}

impl Hero {
    fn get_skill(&self, ability: HeroAbility) -> Option<Level> {
        self.skills.iter().find(|(a, l)| *a == ability).map(|(a, l)| *l)
    }
    fn has_artifact(&self, artifact: Artifact) -> bool {
        self.artifacts.iter().find(|&&a| a == artifact).is_some()
    }
}

fn calculate_strike_damage(
    attacker_hero: Hero,
    attacker: CreatureStack,
    defender_hero: Hero,
    defender: CreatureStack,
    strike_type: StrikeType) -> u32 {
    let (damage_min, damage_max) = attacker.base_stats().damage;
    let (damage_min, damage_max) = (damage_min as f32, damage_max as f32);
    
    // Базовый урон
    let base_damage_single =
        if let Some(blessing) = attacker.get_effect(Effect::Bless) {
            if blessing.level() > Level::Basic {
                damage_max + 1.0
            } else {
                damage_max
            }
        } else if let Some(curse) = attacker.get_effect(Effect::Curse) {
           if curse.level() > Level::Basic {
               damage_min - 1.0
           } else {
               damage_min
           }
        } else {
            (damage_max + damage_min) / 2.0
        };
    
    let base_damage = base_damage_single * attacker.count() as f32;
    dbg!(base_damage);

    // Эффекты, модифицирующие атаку атакующего
    let mut current_attack = attacker.base_stats().attack as f32;
    if let Some(bloodlust) = attacker.get_effect(Effect::Bloodlust) {
        current_attack +=
            if bloodlust.level() == Level::Basic {
                3.0
            } else {
                6.0
            };
    };
    if let Some(frenzy) = attacker.get_effect(Effect::Frenzy) {
        let defence_multiplier =
            match frenzy.level() {
                Level::Basic => 1.0,
                Level::Advanced => 1.5,
                Level::Expert => 2.0
            };
        current_attack += attacker.base_stats().defence as f32 * defence_multiplier;
    }
    if let Some(slayer) = attacker.get_effect(Effect::Slayer) {
        let mut affected_creatures = vec![
            Creature::Behemoth, Creature::AncientBehemoth,
            Creature::Hydra, Creature::ChaosHydra,
            Creature::Firebird, Creature::Phoenix,
            Creature::GreenDragon, Creature::GoldDragon,
            Creature::RedDragon, Creature::BlackDragon,
            Creature::BoneDragon, Creature::GhostDragon,
            Creature::AzureDragon, Creature::CrystalDragon,
            Creature::FaerieDragon, Creature::RustDragon
        ];
        if slayer.level() >= Level::Advanced {
            affected_creatures.extend(&[
                Creature::Angel, Creature::Archangel,
                Creature::Devil, Creature::ArchDevil
            ]);
        }
        if slayer.level() == Level::Expert {
            affected_creatures.extend(&[Creature::Giant, Creature::Titan]);
        }

        if affected_creatures.contains(&defender.creature()) {
            current_attack += 7.0;
        }
    }

    let effect = match strike_type {
            StrikeType::Melee => Effect::Bloodlust,
            StrikeType::Ranged => Effect::Precision
        };
    if let Some(effect) = attacker.get_effect(effect) {
        current_attack +=
            if effect.level() == Level::Basic {
                3.0
            } else {
                6.0
            };
    }

    // Должен быть последним в списке модификаторов
    // чтобы не опустить ниже нуля итоговую атаку
    if let Some(weakness) = attacker.get_effect(Effect::Weakness) {
        let value =
            if weakness.level() == Level::Basic {
                3.0
            } else {
                6.0
            };
        current_attack = f32::max(current_attack - value, 0.0);
    }

    // Эффекты, модифицирующие защиту защищающегося
    let mut current_defence = defender.base_stats().defence as f32;
    if let Some(stoneskin) = defender.get_effect(Effect::StoneSkin) {
        current_defence +=
            if stoneskin.level() == Level::Basic {
                3.0
            } else {
                6.0
            };
    }

    // Разрушающий луч и кислотное дыхание (от Ржавых Драконов)
    // имеют одну и ту же механику
    let stackable_defence_eaters = defender.applied_effects
        .iter()
        .filter(|e| [Effect::DisruptingRay, Effect::AcidBreath].contains(&e.effect()))
        .map(|e| {
            match e.level() {
                Level::Basic => 3, // Кислотное дыхание бывает только Базовое
                Level::Advanced => 4,
                Level::Expert => 5
            }
        })
        .sum::<u8>();
    current_defence -= stackable_defence_eaters as f32;

    let ability = attacker.get_ability(CreatureAbility::IgnoreDefence { percent: 0 });
    if let Some(CreatureAbility::IgnoreDefence {percent: pcnt}) = ability {
        current_defence -= current_defence * (pcnt as f32 / 100.0) + 1.0;
    }

    // текущие значения атаки и защиты
    dbg!(current_attack);
    dbg!(current_defence);

    // основной модификатор урона в двух видах
    let md_1 = 
        if current_attack > current_defence {
            0.05 * (current_attack - current_defence)
        } else {
            0.0
        };
    let md_1 = f32::min(md_1, 3.0);
    dbg!(md_1);

    let md_2 =
        if current_defence > current_attack {
            1.0 - 0.025 * (current_defence - current_attack)
        } else {
            1.0
        };
    let md_2 = f32::max(md_2, 0.3);
    dbg!(md_2);

    // Модификатор вторичного навыка Нападение/Стрельба
    let m_off =
        match strike_type {
            StrikeType::Melee => {
                if let Some(level) = attacker_hero.get_skill(HeroAbility::Offense) {
                    match level {
                        Level::Basic => 0.1,
                        Level::Advanced => 0.2,
                        Level::Expert => 0.3
                    }
                } else {
                    0.0
                }
            },
            StrikeType::Ranged => {
                if let Some(level) = attacker_hero.get_skill(HeroAbility::Archery) {
                    let base_modifier =
                        match level {
                            Level::Basic => 0.1,
                            Level::Advanced => 0.25,
                            Level::Expert => 0.5
                        };
                    let artifacts = [
                        (Artifact::BowOfElvenCherrywood, 0.05),
                        (Artifact::BowstringOfTheUnicorn, 0.1),
                        (Artifact::AngelFeatherArrows, 0.15),
                        (Artifact::BowOfTheSharpshooter, 0.3)
                    ];

                    let artifacts_modifier =
                        artifacts
                            .iter()
                            .filter(|(art, _val)| attacker_hero.has_artifact(*art))
                            .map(|(_art, val)| val)
                            .sum::<f32>();

                    base_modifier + artifacts_modifier
                } else {
                    0.0
                }
            }
        };
    
    let damage = base_damage * (1.0 + md_1 + m_off) * md_2;
    damage.round() as u32
}

fn main() {

    let attacker_hero = Hero {
        attack: 1,
        defence: 1,
        spell_power: 1,
        knowledge: 1,
        skills: vec![(HeroAbility::Offense, Level::Basic)],
        artifacts: vec![]
    };

    let defender_hero = Hero {
        attack: 1,
        defence: 10,
        spell_power: 1,
        knowledge: 1,
        skills: vec![(HeroAbility::Armorer, Level::Expert)],
        artifacts: vec![]
    };

    let mut attacker = CreatureStack::new(Creature::Behemoth, 10);
    attacker.apply_effect(Effect::Bless, Level::Basic);

    let mut defender = CreatureStack::new(Creature::Demon, 100);
    defender.apply_effect(Effect::StoneSkin, Level::Basic);

    let final_damage = calculate_strike_damage(attacker_hero, attacker, defender_hero, defender, StrikeType::Melee);
    dbg!(final_damage);
}
