#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Effect {
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Level {
    Basic,
    Advanced,
    Expert,
}

pub struct AppliedEffect {
    effect: Effect,
    level: Level,
}

impl AppliedEffect {
    pub fn new(effect: Effect, level: Level) -> Self {
        Self { effect, level }
    }
    pub fn effect(&self) -> Effect {
        self.effect
    }
    pub fn level(&self) -> Level {
        self.level
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum HeroAbility {
    Offense,
    Archery,
    Armorer
}

#[derive(PartialEq, Clone, Copy)]
pub enum Artifact {
    BowOfElvenCherrywood,
    BowstringOfTheUnicorn,
    AngelFeatherArrows,
    BowOfTheSharpshooter
}

#[derive(PartialEq, Clone, Copy)]
pub enum Specialty {
    HeroAbility(HeroAbility),
    Spell(Effect) // TODO: разделить Effects и Spells
}