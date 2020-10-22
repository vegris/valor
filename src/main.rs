#![allow(unused)]

use std::cmp::{min, max};

mod creatures;
mod creature_stack;
mod skills;
mod action_queue;

use creatures::{Creature, CreatureAbility};
use creature_stack::{CreatureStack, GridPos};
use skills::{Effect, Level, AppliedEffect, HeroAbility, Artifact, Specialty};
use action_queue::{Action, ActionQueue};

#[derive(PartialEq)]
enum StrikeType {
    Melee,
    Ranged
}

struct Hero {
    level: u8,

    attack: u8,
    defence: u8,
    spell_power: u8,
    knowledge: u8,
    
    specialty: Specialty,
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

#[derive(PartialEq)]
enum WallCondition {
    New,
    Damaged,
    Destroyed
}
struct Wall {
    position: GridPos,
    condition: WallCondition
}

fn calculate_strike_damage(
    attacker_hero: Hero,
    attacker: CreatureStack,
    defender_hero: Hero,
    defender: CreatureStack,
    strike_type: StrikeType,
    action_queue: ActionQueue,
    walls: Option<&Vec<Wall>>) -> u32 {

    let (damage_min, damage_max) = attacker.base_stats().damage;
    let (damage_min, damage_max) = (damage_min as u32, damage_max as u32);
    
    // Базовый урон
    let base_damage_single =
        if let Some(blessing) = attacker.get_effect(Effect::Bless) {
            if blessing.level() > Level::Basic {
                damage_max + 1
            } else {
                damage_max
            }
        } else if let Some(curse) = attacker.get_effect(Effect::Curse) {
           if curse.level() > Level::Basic {
               damage_min - 1
           } else {
               damage_min
           }
        } else {
            (damage_max + damage_min) / 2
        };
    
    let base_damage = base_damage_single * attacker.count();
    dbg!(base_damage);

    // Эффекты, модифицирующие атаку атакующего
    let mut current_attack = (attacker.base_stats().attack + attacker_hero.attack) as u32;
    if let Some(bloodlust) = attacker.get_effect(Effect::Bloodlust) {
        current_attack +=
            if bloodlust.level() == Level::Basic {
                3
            } else {
                6
            };
    };
    if let Some(frenzy) = attacker.get_effect(Effect::Frenzy) {
        let defence_multiplier =
            match frenzy.level() {
                Level::Basic => 1.0,
                Level::Advanced => 1.5,
                Level::Expert => 2.0
            };
        current_attack += (attacker.base_stats().defence as f32 * defence_multiplier).round() as u32;
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
            current_attack += 7;
        }
    }

    let effect = match strike_type {
            StrikeType::Melee => Effect::Bloodlust,
            StrikeType::Ranged => Effect::Precision
        };
    if let Some(effect) = attacker.get_effect(effect) {
        current_attack +=
            if effect.level() == Level::Basic {
                3
            } else {
                6
            };
    }

    // Должен быть последним в списке модификаторов
    // чтобы не опустить ниже нуля итоговую атаку
    if let Some(weakness) = attacker.get_effect(Effect::Weakness) {
        let value =
            if weakness.level() == Level::Basic {
                3
            } else {
                6
            };
        current_attack = max(current_attack - value, 0);
    }

    // Эффекты, модифицирующие защиту защищающегося
    let mut current_defence = (defender.base_stats().defence + defender_hero.defence) as u32;
    if let Some(stoneskin) = defender.get_effect(Effect::StoneSkin) {
        current_defence +=
            if stoneskin.level() == Level::Basic {
                3
            } else {
                6
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
        .sum::<u32>();
    current_defence -= stackable_defence_eaters;

    let ability = attacker.get_ability(CreatureAbility::IgnoreDefence { percent: 0 });
    if let Some(CreatureAbility::IgnoreDefence {percent: pcnt}) = ability {
        current_defence -= (current_defence as f32 * (pcnt as f32 / 100.0)) as u32 + 1;
    }

    // текущие значения атаки и защиты
    dbg!(current_attack);
    dbg!(current_defence);

    // основной модификатор урона в двух видах
    let md_1 = 
        if current_attack > current_defence {
            0.05 * (current_attack - current_defence) as f32
        } else {
            0.0
        };
    let md_1 = f32::min(md_1, 3.0);
    dbg!(md_1);

    let md_2 =
        if current_defence > current_attack {
            1.0 - 0.025 * (current_defence - current_attack) as f32
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
    dbg!(m_off);
    
    // Модификатор специализации 
    let m_spec =
        if attacker_hero.specialty == Specialty::Spell(Effect::Bless) {
            0.03 * (attacker_hero.level as f32 / attacker.base_stats().level as f32).floor()
        } else if [
                Specialty::HeroAbility(HeroAbility::Offense),
                Specialty::HeroAbility(HeroAbility::Archery)
            ].contains(&attacker_hero.specialty) {
                0.05 * attacker_hero.level as f32
        } else {
            0.0
        };
    dbg!(m_spec);

    // Модификатор удачи
    let m_luck = action_queue.has_proc(Effect::Luck) as u8 as f32;
    dbg!(m_luck);

    let m_at =
        if attacker.has_ability(CreatureAbility::CavalierBonus) {
            0.05 * action_queue.cells_walked() as f32
        } else if action_queue.has_proc(Effect::DeathBlow) {
            1.0
        } else if let Some(CreatureAbility::Hatred { creature: c, value: v }) =
            attacker.get_ability(CreatureAbility::Hatred { creature: attacker.creature(), value: 0.0}) {
            if c == defender.creature() { v } else { 0.0 }
        } else {
            0.0
        };
    dbg!(m_at);

    // Модификатор доспехов
    let armorer_bonus =
        defender_hero.get_skill(HeroAbility::Armorer)
            .map(|level| {
                match level {
                    Level::Basic => 0.05,
                    Level::Advanced => 0.1,
                    Level::Expert => 0.15
                }
            })
            .map(|bonus| {
                if defender_hero.specialty == Specialty::HeroAbility(HeroAbility::Armorer) {
                    bonus * (1.0 + 0.05 * defender_hero.level as f32)
                } else {
                    bonus
                }
            })
            .unwrap_or(0.0);
    let m_armor = 1.0 - armorer_bonus;
    dbg!(m_armor);

    // Модификатор защитных заклинаний
    let mut m_spell =
        match strike_type {
            StrikeType::Melee => {
                let base = 1.0;
                let modif =
                    if let Some(shield) = defender.get_effect(Effect::Shield) {
                        if shield.level() == Level::Basic {
                            0.15
                        } else {
                            0.3
                        }
                    } else {
                        0.0
                    };
                base - modif
            },
            StrikeType::Ranged => {
                let base =
                    attacker
                        .get_effect(Effect::Forgetfulness)
                        .filter(|e| e.level() == Level::Basic)
                        .and(Some(0.5))
                        .unwrap_or(1.0);
                let modif =
                    if let Some(shield) = defender.get_effect(Effect::AirShield) {
                        if shield.level() == Level::Basic {
                            0.25
                        } else {
                            0.5
                        }
                    } else {
                        0.0
                    };
                base - modif
            }
        };
    dbg!(m_spell);

    let mut m_arch_penalty = 1.0;
        if strike_type == StrikeType::Ranged {
            let arrow_path = attacker.position().get_path_to(&defender.position()).unwrap();
            if arrow_path.len() > 10 {
                m_arch_penalty *= 0.5;
            }
            if !attacker.has_ability(CreatureAbility::NoObstaclePenalty) && walls.is_some() {
                let standing = walls
                    .unwrap()
                    .iter()
                    .filter(|w| w.condition != WallCondition::Destroyed)
                    .map(|w| w.position)
                    .collect::<Vec<GridPos>>();
                if arrow_path.iter().any(|a| standing.contains(a)) {
                    m_arch_penalty *= 0.5;
                }
            }
        } else if attacker.creature().is_ranged() && !attacker.has_ability(CreatureAbility::NoMeleePenalty) {
            m_arch_penalty *= 0.5
        };

    let damage = base_damage as f32 * (1.0 + md_1 + m_off + m_spec + m_luck + m_at) * md_2 * m_armor * m_spell * m_arch_penalty;
    damage.round() as u32
}

fn main() {
    let attacker_hero = Hero {
        level: 5,

        attack: 1,
        defence: 1,
        spell_power: 1,
        knowledge: 1,
        specialty: Specialty::HeroAbility(HeroAbility::Offense),

        skills: vec![(HeroAbility::Offense, Level::Basic)],
        artifacts: vec![]
    };

    let defender_hero = Hero {
        level: 5,

        attack: 1,
        defence: 10,
        spell_power: 1,
        knowledge: 1,

        specialty: Specialty::HeroAbility(HeroAbility::Armorer),
        skills: vec![(HeroAbility::Armorer, Level::Expert)],
        artifacts: vec![]
    };

    let mut attacker = CreatureStack::new(Creature::Champion, 10);
    attacker.apply_effect(Effect::Bless, Level::Basic);

    let mut defender = CreatureStack::new(Creature::Devil, 100);
    defender.apply_effect(Effect::Shield, Level::Basic);

    let action_queue = vec![
        Action::Move((8, 8)),
        Action::Move((8, 8)),
        Action::Move((8, 8)),
        Action::Move((8, 8)),
        Action::Proc(Effect::Luck),
    ].into();

    let final_damage = calculate_strike_damage(attacker_hero, attacker, defender_hero, defender, StrikeType::Melee, action_queue, None);
    dbg!(final_damage);
}
