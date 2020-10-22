use super::creatures::{Creature, CreatureStats, CreatureAbility};
use super::skills::{AppliedEffect, Effect, Level};

pub struct CreatureStack {
    creature: Creature,
    count: u32,
    current_health: u16,
    current_ammo: u8,
    position: GridPos,
    pub applied_effects: Vec<AppliedEffect>,
}

impl CreatureStack {
    pub fn new(creature: Creature, count: u32) -> Self {
        CreatureStack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            position: GridPos::new(1, 1),
            applied_effects: Vec::new(),
        }
    }
    pub fn base_stats(&self) -> CreatureStats {
        self.creature.base_stats()
    }

    pub fn creature(&self) -> Creature {
        self.creature
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn position(&self) -> GridPos {
        self.position
    }

    pub fn get_effect(&self, effect: Effect) -> Option<&AppliedEffect> {
        self.applied_effects.iter().find(|&x| x.effect() == effect)
    }

    pub fn apply_effect(&mut self, effect: Effect, level: Level) {
        self.applied_effects.push(AppliedEffect::new(effect, level));
    }

    pub fn get_ability(&self, ability: CreatureAbility) -> Option<CreatureAbility> {
        self.creature.abilities().into_iter().find(|a| *a == ability)
    }

    pub fn has_ability(&self, ability: CreatureAbility) -> bool {
        self.get_ability(ability).is_some()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct GridPos {
    x: u8,
    y: u8
}

impl GridPos {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    // TODO: placeholder
    pub fn get_path_to(&self, other: &Self) -> Option<Vec<Self>> {
        Some(vec![Self::new(1, 1)])
    }
}