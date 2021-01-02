#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum SkillLevel {
    Basic,
    Advanced,
    Expert
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum Spell {
    Bless,
    Curse,
    Frenzy,
    Slayer,
    Bloodlust,
    Precision,
    Weakness,
    StoneSkin,
    DisruptingRay,
    AcidBreath,
    Luck,
    DeathBlow,
    Shield,
    AirShield,
    Forgetfulness,
    Blinded,
    TurnedToStone,
    Paralyzed
}

#[derive(Debug)]
pub struct AppliedSpell {
    spell: Spell,
    level: SkillLevel,
}

impl AppliedSpell {
    pub fn new(spell: Spell, level: SkillLevel) -> Self {
        Self { spell, level }
    }
    pub fn spell(&self) -> Spell {
        self.spell
    }
    pub fn level(&self) -> SkillLevel {
        self.level
    }
}
