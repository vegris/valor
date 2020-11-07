use super::creature::{Creature, CreatureAbility, CreatureStats};
use super::skills::{Spell, AppliedSpell, SkillLevel};

/// Существо в течение раунда может принимать одно из этих состояний
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreatureTurnState {
    HasTurn,
    MoraledAndWaited,
    Waited,
    NoTurn
}

pub struct CreatureStack {
    creature: Creature,
    count: u32,

    current_health: u16,
    current_ammo: u8,

    position: GridPos,

    applied_spells: Vec<AppliedSpell>,

    pub turn_state: CreatureTurnState,
    pub defending: bool
}

impl CreatureStack {
    pub fn new(creature: Creature, count: u32) -> Self {
        CreatureStack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            position: GridPos::new(1, 1),
            applied_spells: Vec::new(),
            turn_state: CreatureTurnState::HasTurn,
            defending: false
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
    pub fn speed(&self) -> u8 {
        self.creature.base_stats().speed
    }
    pub fn applied_effects(&self) -> &Vec<AppliedSpell> {
        &self.applied_spells
    }

    pub fn get_effect(&self, spell: Spell) -> Option<&AppliedSpell> {
        self.applied_spells.iter().find(|&x| x.spell() == spell)
    }

    pub fn apply_effect(&mut self, spell: Spell, level: SkillLevel) {
        self.applied_spells.push(AppliedSpell::new(spell, level));
    }

    pub fn get_ability(&self, ability: CreatureAbility) -> Option<CreatureAbility> {
        self.creature.abilities().iter().find(|&&a| a == ability).copied()
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
    pub fn path_to(self, other: Self) -> Option<Vec<Self>> {
        Some(vec![Self::new(1, 1)])
    }
}