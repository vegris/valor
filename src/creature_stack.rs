use std::collections::HashSet;

use super::creature::{Creature, CreatureAbility, CreatureStats};
use super::skills::{Spell, AppliedSpell, SkillLevel};
use super::gridpos::GridPos;
use super::battlestate::Side;

/// Существо в течение раунда может принимать одно из этих состояний
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreatureTurnState {
    HasTurn,
    Waited,
    NoTurn
}

#[derive(Debug)]
pub struct CreatureStack {
    creature: Creature,
    count: u32,

    current_health: u16,
    pub current_ammo: u8,

    position: GridPos,

    applied_spells: Vec<AppliedSpell>,

    pub turn_state: CreatureTurnState,
    pub defending: bool
}

impl CreatureStack {
    pub fn new(creature: Creature, count: u32, position: GridPos) -> Self {
        CreatureStack {
            creature,
            count,
            current_health: creature.base_stats().health,
            current_ammo: creature.base_stats().ammo_capacity,
            position,
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
    pub fn set_position(&mut self, pos: GridPos) {
        self.position = pos;
    }
    pub fn speed(&self) -> u8 {
        self.creature.base_stats().speed
    }
    pub fn applied_effects(&self) -> &Vec<AppliedSpell> {
        &self.applied_spells
    }

    pub fn is_alive(&self) -> bool {
        self.count == 0
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

    pub fn get_occupied_cells(&self, side: Side) -> Vec<GridPos> {
        if self.creature.is_wide() {
            let second_cell =
                match side {
                    Side::Attacker => self.position.relative(0, 1),
                    Side::Defender => self.position.relative(0, -1)
                };
            vec![self.position, second_cell]
        } else {
            vec![self.position]
        }
    }

    pub fn get_adjacent_cells(&self, side: Side) -> Vec<GridPos> {
        self.get_occupied_cells(side)
            .iter()
            .map(|cell| cell.get_successors())
            .flatten()
            .collect::<HashSet<GridPos>>() // Оставляем уникальные
            .drain()
            .collect::<Vec<GridPos>>()
    }

    pub fn receive_damage(&mut self, damage: u32) {
        let unit_health = self.base_stats().health;
        let total_health = (self.count - 1) * unit_health as u32 + self.current_health as u32;
        if total_health <= damage {
            self.current_health = 0;
            self.count = 0;
        } else {
            let health_left = total_health - damage;
            let creatures_left = health_left / unit_health as u32;
            let current_health = health_left as u16 % unit_health;

            if current_health == 0 {
                self.count = creatures_left - 1;
                self.current_health = unit_health;
            } else {
                self.count = creatures_left;
                self.current_health = current_health;
            }
        }
    }
}

use std::fmt;
impl fmt::Display for CreatureStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.creature, self.count)
    }
}
