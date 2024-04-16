use strum_macros::{EnumCount, EnumIter};

use crate::heroes::AbilityLevel;

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

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum SpellAnimation {
    Armageddon,
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

    pub const fn cost(self, school_level: Option<AbilityLevel>) -> i32 {
        match (self, school_level) {
            (Spell::MagicArrow, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::MagicArrow, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::LightningBolt, None | Some(AbilityLevel::Basic)) => 10,
            (Spell::LightningBolt, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 8,

            (Spell::DestroyUndead, None | Some(AbilityLevel::Basic)) => 15,
            (Spell::DestroyUndead, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::ChainLightning, None | Some(AbilityLevel::Basic)) => 24,
            (Spell::ChainLightning, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::TitanLightningBolt, _) => 0,

            (Spell::DeathRipple, None | Some(AbilityLevel::Basic)) => 10,
            (Spell::DeathRipple, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 8,

            (Spell::MeteorShower, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::MeteorShower, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::Implosion, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::Implosion, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::FireWall, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::FireWall, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 6,

            (Spell::Fireball, None | Some(AbilityLevel::Basic)) => 15,
            (Spell::Fireball, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::LandMine, None | Some(AbilityLevel::Basic)) => 18,
            (Spell::LandMine, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 15,

            (Spell::Armageddon, None | Some(AbilityLevel::Basic)) => 24,
            (Spell::Armageddon, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Inferno, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::Inferno, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::IceBolt, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::IceBolt, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 6,

            (Spell::FrostRing, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::FrostRing, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::Haste, None | Some(AbilityLevel::Basic)) => 6,
            (Spell::Haste, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::DisruptingRay, None | Some(AbilityLevel::Basic)) => 10,
            (Spell::DisruptingRay, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 8,

            (Spell::Fortune, None | Some(AbilityLevel::Basic)) => 7,
            (Spell::Fortune, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::Precision, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::Precision, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 6,

            (Spell::ProtectAir, None | Some(AbilityLevel::Basic)) => 7,
            (Spell::ProtectAir, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::AirShield, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::AirShield, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::Hypnotize, None | Some(AbilityLevel::Basic)) => 18,
            (Spell::Hypnotize, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 15,

            (Spell::Counterstrike, None | Some(AbilityLevel::Basic)) => 24,
            (Spell::Counterstrike, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::MagicMirror, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::MagicMirror, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::SummonAir, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::SummonAir, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Shield, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::Shield, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::Slow, None | Some(AbilityLevel::Basic)) => 6,
            (Spell::Slow, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::StoneSkin, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::StoneSkin, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::Quicksand, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::Quicksand, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 6,

            (Spell::AnimateDead, None | Some(AbilityLevel::Basic)) => 15,
            (Spell::AnimateDead, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::AntiMagic, None | Some(AbilityLevel::Basic)) => 15,
            (Spell::AntiMagic, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::Earthquake, None | Some(AbilityLevel::Basic)) => 20,
            (Spell::Earthquake, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 17,

            (Spell::ForceField, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::ForceField, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::ProtectEarth, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::ProtectEarth, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::Resurrection, None | Some(AbilityLevel::Basic)) => 20,
            (Spell::Resurrection, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 16,

            (Spell::Sorrow, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::Sorrow, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::SummonEarth, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::SummonEarth, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Bloodlust, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::Bloodlust, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::Curse, None | Some(AbilityLevel::Basic)) => 6,
            (Spell::Curse, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::ProtectFire, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::ProtectFire, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::Blind, None | Some(AbilityLevel::Basic)) => 10,
            (Spell::Blind, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 8,

            (Spell::Misfortune, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::Misfortune, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::Berserk, None | Some(AbilityLevel::Basic)) => 20,
            (Spell::Berserk, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 16,

            (Spell::FireShield, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::FireShield, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::Frenzy, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::Frenzy, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::Slayer, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::Slayer, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::Sacrifice, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::Sacrifice, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::SummonFire, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::SummonFire, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Bless, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::Bless, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::Cure, None | Some(AbilityLevel::Basic)) => 6,
            (Spell::Cure, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::Dispel, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::Dispel, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::ProtectWater, None | Some(AbilityLevel::Basic)) => 5,
            (Spell::ProtectWater, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 4,

            (Spell::RemoveObstacle, None | Some(AbilityLevel::Basic)) => 7,
            (Spell::RemoveObstacle, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 5,

            (Spell::Weakness, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::Weakness, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 6,

            (Spell::Forgetfulness, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::Forgetfulness, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::Mirth, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::Mirth, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 9,

            (Spell::Teleport, None) => 15,
            (Spell::Teleport, Some(AbilityLevel::Basic)) => 12,
            (Spell::Teleport, Some(AbilityLevel::Advanced)) => 6,
            (Spell::Teleport, Some(AbilityLevel::Expert)) => 3,

            (Spell::Clone, None | Some(AbilityLevel::Basic)) => 24,
            (Spell::Clone, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Prayer, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::Prayer, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::SummonWater, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::SummonWater, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Visions, None | Some(AbilityLevel::Basic)) => 4,
            (Spell::Visions, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 2,

            (Spell::ViewAir, None | Some(AbilityLevel::Basic)) => 2,
            (Spell::ViewAir, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 1,

            (Spell::Disguise, None | Some(AbilityLevel::Basic)) => 4,
            (Spell::Disguise, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 2,

            (Spell::DimensionDoor, None | Some(AbilityLevel::Basic)) => 25,
            (Spell::DimensionDoor, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 20,

            (Spell::Fly, None | Some(AbilityLevel::Basic)) => 20,
            (Spell::Fly, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 15,

            (Spell::ViewEarth, None | Some(AbilityLevel::Basic)) => 2,
            (Spell::ViewEarth, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 1,

            (Spell::TownPortal, None | Some(AbilityLevel::Basic)) => 16,
            (Spell::TownPortal, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 12,

            (Spell::SummonBoat, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::SummonBoat, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 7,

            (Spell::ScuttleBoat, None | Some(AbilityLevel::Basic)) => 8,
            (Spell::ScuttleBoat, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 6,

            (Spell::WaterWalk, None | Some(AbilityLevel::Basic)) => 12,
            (Spell::WaterWalk, Some(AbilityLevel::Advanced | AbilityLevel::Expert)) => 8,
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

impl SpellAnimation {
    pub const SPRITESHEET_TYPE: u32 = 64;

    pub const fn spritesheet(self) -> &'static str {
        match self {
            Self::Armageddon => "C06SPF0.def",
        }
    }
}
