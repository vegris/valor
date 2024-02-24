use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, EnumCount)]
pub enum Spell {
    SummonBoat,
    ScuttleBoat,
    Visions,
    ViewAir,
    Disguise,
    ViewEarth,
    Fly,
    WaterWalk,
    DimensionDoor,
    TownPortal,
    Quicksand,
    LandMine,
    ForceField,
    FireWall,
    Earthquake,
    MagicArrow,
    IceBolt,
    LightningBolt,
    Implosion,
    ChainLightning,
    FrostRing,
    Fireball,
    Inferno,
    MeteorShower,
    DeathRipple,
    DestroyUndead,
    Armageddon,
    Shield,
    AirShield,
    FireShield,
    ProtectAir,
    ProtectFire,
    ProtectWater,
    ProtectEarth,
    AntiMagic,
    Dispel,
    MagicMirror,
    Cure,
    Resurrection,
    AnimateDead,
    Sacrifice,
    Bless,
    Curse,
    Bloodlust,
    Precision,
    Weakness,
    StoneSkin,
    DisruptingRay,
    Prayer,
    Sorrow,
    Fortune,
    Mirth,
    Misfortune,
    Haste,
    Slow,
    Slayer,
    Frenzy,
    TitanLightningBolt,
    Counterstrike,
    Berserk,
    Hypnotize,
    Forgetfulness,
    Blind,
    Teleport,
    RemoveObstacle,
    Clone,
    SummonEarth,
    SummonFire,
    SummonWater,
    SummonAir,
}

#[derive(Clone, Copy)]
pub enum SpellType {
    Adventure,
    Battle,
}

#[derive(Clone, Copy)]
pub enum SpellSchool {
    Air,
    Earth,
    Fire,
    Water,
}

#[derive(Clone, Copy)]
pub enum SpellLevel {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl Spell {
    pub const SPRITESHEET: &'static str = "spells.def";

    pub fn r#type(self) -> SpellType {
        let adventure_spells = [
            Spell::SummonBoat,
            Spell::ScuttleBoat,
            Spell::Visions,
            Spell::ViewAir,
            Spell::Disguise,
            Spell::ViewEarth,
            Spell::Fly,
            Spell::WaterWalk,
            Spell::DimensionDoor,
            Spell::TownPortal,
        ];

        if adventure_spells.contains(&self) {
            SpellType::Adventure
        } else {
            SpellType::Battle
        }
    }

    pub fn school(self) -> Option<SpellSchool> {
        match self {
            Self::MagicArrow => None,
            Self::LightningBolt => Some(SpellSchool::Air),
            Self::DestroyUndead => Some(SpellSchool::Air),
            Self::ChainLightning => Some(SpellSchool::Air),
            Self::TitanLightningBolt => Some(SpellSchool::Air),
            Self::DeathRipple => Some(SpellSchool::Earth),
            Self::MeteorShower => Some(SpellSchool::Earth),
            Self::Implosion => Some(SpellSchool::Earth),
            Self::FireWall => Some(SpellSchool::Fire),
            Self::Fireball => Some(SpellSchool::Fire),
            Self::LandMine => Some(SpellSchool::Fire),
            Self::Armageddon => Some(SpellSchool::Fire),
            Self::Inferno => Some(SpellSchool::Fire),
            Self::IceBolt => Some(SpellSchool::Water),
            Self::FrostRing => Some(SpellSchool::Water),
            Self::Haste => Some(SpellSchool::Air),
            Self::DisruptingRay => Some(SpellSchool::Air),
            Self::Fortune => Some(SpellSchool::Air),
            Self::Precision => Some(SpellSchool::Air),
            Self::ProtectAir => Some(SpellSchool::Air),
            Self::AirShield => Some(SpellSchool::Air),
            Self::Hypnotize => Some(SpellSchool::Air),
            Self::Counterstrike => Some(SpellSchool::Air),
            Self::MagicMirror => Some(SpellSchool::Air),
            Self::SummonAir => Some(SpellSchool::Air),
            Self::Shield => Some(SpellSchool::Earth),
            Self::Slow => Some(SpellSchool::Earth),
            Self::StoneSkin => Some(SpellSchool::Earth),
            Self::Quicksand => Some(SpellSchool::Earth),
            Self::AnimateDead => Some(SpellSchool::Earth),
            Self::AntiMagic => Some(SpellSchool::Earth),
            Self::Earthquake => Some(SpellSchool::Earth),
            Self::ForceField => Some(SpellSchool::Earth),
            Self::ProtectEarth => Some(SpellSchool::Earth),
            Self::Resurrection => Some(SpellSchool::Earth),
            Self::Sorrow => Some(SpellSchool::Earth),
            Self::SummonEarth => Some(SpellSchool::Earth),
            Self::Bloodlust => Some(SpellSchool::Fire),
            Self::Curse => Some(SpellSchool::Fire),
            Self::ProtectFire => Some(SpellSchool::Fire),
            Self::Blind => Some(SpellSchool::Fire),
            Self::Misfortune => Some(SpellSchool::Fire),
            Self::Berserk => Some(SpellSchool::Fire),
            Self::FireShield => Some(SpellSchool::Fire),
            Self::Frenzy => Some(SpellSchool::Fire),
            Self::Slayer => Some(SpellSchool::Fire),
            Self::Sacrifice => Some(SpellSchool::Fire),
            Self::SummonFire => Some(SpellSchool::Fire),
            Self::Bless => Some(SpellSchool::Water),
            Self::Cure => Some(SpellSchool::Water),
            Self::Dispel => Some(SpellSchool::Water),
            Self::ProtectWater => Some(SpellSchool::Water),
            Self::RemoveObstacle => Some(SpellSchool::Water),
            Self::Weakness => Some(SpellSchool::Water),
            Self::Forgetfulness => Some(SpellSchool::Water),
            Self::Mirth => Some(SpellSchool::Water),
            Self::Teleport => Some(SpellSchool::Water),
            Self::Clone => Some(SpellSchool::Water),
            Self::Prayer => Some(SpellSchool::Water),
            Self::SummonWater => Some(SpellSchool::Water),
            Self::Visions => None,
            Self::ViewAir => Some(SpellSchool::Air),
            Self::Disguise => Some(SpellSchool::Air),
            Self::DimensionDoor => Some(SpellSchool::Air),
            Self::Fly => Some(SpellSchool::Air),
            Self::ViewEarth => Some(SpellSchool::Earth),
            Self::TownPortal => Some(SpellSchool::Earth),
            Self::SummonBoat => Some(SpellSchool::Water),
            Self::ScuttleBoat => Some(SpellSchool::Water),
            Self::WaterWalk => Some(SpellSchool::Water),
        }
    }

    pub fn level(self) -> SpellLevel {
        match self {
            Self::MagicArrow => SpellLevel::First,
            Self::LightningBolt => SpellLevel::Second,
            Self::DestroyUndead => SpellLevel::Third,
            Self::ChainLightning => SpellLevel::Fourth,
            Self::TitanLightningBolt => SpellLevel::Fifth,
            Self::DeathRipple => SpellLevel::Second,
            Self::MeteorShower => SpellLevel::Fourth,
            Self::Implosion => SpellLevel::Fifth,
            Self::FireWall => SpellLevel::Second,
            Self::Fireball => SpellLevel::Third,
            Self::LandMine => SpellLevel::Third,
            Self::Armageddon => SpellLevel::Fourth,
            Self::Inferno => SpellLevel::Fourth,
            Self::IceBolt => SpellLevel::Second,
            Self::FrostRing => SpellLevel::Third,
            Self::Haste => SpellLevel::First,
            Self::DisruptingRay => SpellLevel::Second,
            Self::Fortune => SpellLevel::Second,
            Self::Precision => SpellLevel::Second,
            Self::ProtectAir => SpellLevel::Second,
            Self::AirShield => SpellLevel::Third,
            Self::Hypnotize => SpellLevel::Third,
            Self::Counterstrike => SpellLevel::Fourth,
            Self::MagicMirror => SpellLevel::Fifth,
            Self::SummonAir => SpellLevel::Fifth,
            Self::Shield => SpellLevel::First,
            Self::Slow => SpellLevel::First,
            Self::StoneSkin => SpellLevel::First,
            Self::Quicksand => SpellLevel::Second,
            Self::AnimateDead => SpellLevel::Third,
            Self::AntiMagic => SpellLevel::Third,
            Self::Earthquake => SpellLevel::Third,
            Self::ForceField => SpellLevel::Third,
            Self::ProtectEarth => SpellLevel::Third,
            Self::Resurrection => SpellLevel::Fourth,
            Self::Sorrow => SpellLevel::Fourth,
            Self::SummonEarth => SpellLevel::Fifth,
            Self::Bloodlust => SpellLevel::First,
            Self::Curse => SpellLevel::First,
            Self::ProtectFire => SpellLevel::First,
            Self::Blind => SpellLevel::Second,
            Self::Misfortune => SpellLevel::Third,
            Self::Berserk => SpellLevel::Fourth,
            Self::FireShield => SpellLevel::Fourth,
            Self::Frenzy => SpellLevel::Fourth,
            Self::Slayer => SpellLevel::Fourth,
            Self::Sacrifice => SpellLevel::Fifth,
            Self::SummonFire => SpellLevel::Fifth,
            Self::Bless => SpellLevel::First,
            Self::Cure => SpellLevel::First,
            Self::Dispel => SpellLevel::First,
            Self::ProtectWater => SpellLevel::First,
            Self::RemoveObstacle => SpellLevel::Second,
            Self::Weakness => SpellLevel::Second,
            Self::Forgetfulness => SpellLevel::Third,
            Self::Mirth => SpellLevel::Third,
            Self::Teleport => SpellLevel::Third,
            Self::Clone => SpellLevel::Fourth,
            Self::Prayer => SpellLevel::Fourth,
            Self::SummonWater => SpellLevel::Fifth,
            Self::Visions => SpellLevel::Second,
            Self::ViewAir => SpellLevel::First,
            Self::Disguise => SpellLevel::Second,
            Self::DimensionDoor => SpellLevel::Fifth,
            Self::Fly => SpellLevel::Fifth,
            Self::ViewEarth => SpellLevel::First,
            Self::TownPortal => SpellLevel::Fourth,
            Self::SummonBoat => SpellLevel::First,
            Self::ScuttleBoat => SpellLevel::Second,
            Self::WaterWalk => SpellLevel::Fourth,
        }
    }
}

impl SpellSchool {
    pub fn spritesheet(self) -> &'static str {
        match self {
            Self::Air => "SpLevA.def",
            Self::Earth => "SpLevE.def",
            Self::Fire => "SpLevF.def",
            Self::Water => "SpLevW.def",
        }
    }
}

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum SpellAnimation {
    Armageddon,
}

impl SpellAnimation {
    pub const fn spritesheet(self) -> &'static str {
        match self {
            Self::Armageddon => "C06SPF0.def",
        }
    }
}
