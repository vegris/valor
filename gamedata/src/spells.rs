use strum_macros::{EnumCount, EnumIter};

use crate::heroes::abilities::Level;

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

    pub const fn cost(self, school_level: Option<Level>) -> i32 {
        match (self, school_level) {
            (Spell::MagicArrow, None | Some(Level::Basic)) => 5,
            (Spell::MagicArrow, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::LightningBolt, None | Some(Level::Basic)) => 10,
            (Spell::LightningBolt, Some(Level::Advanced | Level::Expert)) => 8,

            (Spell::DestroyUndead, None | Some(Level::Basic)) => 15,
            (Spell::DestroyUndead, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::ChainLightning, None | Some(Level::Basic)) => 24,
            (Spell::ChainLightning, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::TitanLightningBolt, _) => 0,

            (Spell::DeathRipple, None | Some(Level::Basic)) => 10,
            (Spell::DeathRipple, Some(Level::Advanced | Level::Expert)) => 8,

            (Spell::MeteorShower, None | Some(Level::Basic)) => 16,
            (Spell::MeteorShower, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::Implosion, None | Some(Level::Basic)) => 25,
            (Spell::Implosion, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::FireWall, None | Some(Level::Basic)) => 8,
            (Spell::FireWall, Some(Level::Advanced | Level::Expert)) => 6,

            (Spell::Fireball, None | Some(Level::Basic)) => 15,
            (Spell::Fireball, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::LandMine, None | Some(Level::Basic)) => 18,
            (Spell::LandMine, Some(Level::Advanced | Level::Expert)) => 15,

            (Spell::Armageddon, None | Some(Level::Basic)) => 24,
            (Spell::Armageddon, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Inferno, None | Some(Level::Basic)) => 16,
            (Spell::Inferno, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::IceBolt, None | Some(Level::Basic)) => 8,
            (Spell::IceBolt, Some(Level::Advanced | Level::Expert)) => 6,

            (Spell::FrostRing, None | Some(Level::Basic)) => 12,
            (Spell::FrostRing, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::Haste, None | Some(Level::Basic)) => 6,
            (Spell::Haste, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::DisruptingRay, None | Some(Level::Basic)) => 10,
            (Spell::DisruptingRay, Some(Level::Advanced | Level::Expert)) => 8,

            (Spell::Fortune, None | Some(Level::Basic)) => 7,
            (Spell::Fortune, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::Precision, None | Some(Level::Basic)) => 8,
            (Spell::Precision, Some(Level::Advanced | Level::Expert)) => 6,

            (Spell::ProtectAir, None | Some(Level::Basic)) => 7,
            (Spell::ProtectAir, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::AirShield, None | Some(Level::Basic)) => 12,
            (Spell::AirShield, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::Hypnotize, None | Some(Level::Basic)) => 18,
            (Spell::Hypnotize, Some(Level::Advanced | Level::Expert)) => 15,

            (Spell::Counterstrike, None | Some(Level::Basic)) => 24,
            (Spell::Counterstrike, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::MagicMirror, None | Some(Level::Basic)) => 25,
            (Spell::MagicMirror, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::SummonAir, None | Some(Level::Basic)) => 25,
            (Spell::SummonAir, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Shield, None | Some(Level::Basic)) => 5,
            (Spell::Shield, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::Slow, None | Some(Level::Basic)) => 6,
            (Spell::Slow, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::StoneSkin, None | Some(Level::Basic)) => 5,
            (Spell::StoneSkin, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::Quicksand, None | Some(Level::Basic)) => 8,
            (Spell::Quicksand, Some(Level::Advanced | Level::Expert)) => 6,

            (Spell::AnimateDead, None | Some(Level::Basic)) => 15,
            (Spell::AnimateDead, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::AntiMagic, None | Some(Level::Basic)) => 15,
            (Spell::AntiMagic, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::Earthquake, None | Some(Level::Basic)) => 20,
            (Spell::Earthquake, Some(Level::Advanced | Level::Expert)) => 17,

            (Spell::ForceField, None | Some(Level::Basic)) => 12,
            (Spell::ForceField, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::ProtectEarth, None | Some(Level::Basic)) => 12,
            (Spell::ProtectEarth, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::Resurrection, None | Some(Level::Basic)) => 20,
            (Spell::Resurrection, Some(Level::Advanced | Level::Expert)) => 16,

            (Spell::Sorrow, None | Some(Level::Basic)) => 16,
            (Spell::Sorrow, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::SummonEarth, None | Some(Level::Basic)) => 25,
            (Spell::SummonEarth, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Bloodlust, None | Some(Level::Basic)) => 5,
            (Spell::Bloodlust, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::Curse, None | Some(Level::Basic)) => 6,
            (Spell::Curse, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::ProtectFire, None | Some(Level::Basic)) => 5,
            (Spell::ProtectFire, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::Blind, None | Some(Level::Basic)) => 10,
            (Spell::Blind, Some(Level::Advanced | Level::Expert)) => 8,

            (Spell::Misfortune, None | Some(Level::Basic)) => 12,
            (Spell::Misfortune, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::Berserk, None | Some(Level::Basic)) => 20,
            (Spell::Berserk, Some(Level::Advanced | Level::Expert)) => 16,

            (Spell::FireShield, None | Some(Level::Basic)) => 16,
            (Spell::FireShield, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::Frenzy, None | Some(Level::Basic)) => 16,
            (Spell::Frenzy, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::Slayer, None | Some(Level::Basic)) => 16,
            (Spell::Slayer, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::Sacrifice, None | Some(Level::Basic)) => 25,
            (Spell::Sacrifice, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::SummonFire, None | Some(Level::Basic)) => 25,
            (Spell::SummonFire, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Bless, None | Some(Level::Basic)) => 5,
            (Spell::Bless, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::Cure, None | Some(Level::Basic)) => 6,
            (Spell::Cure, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::Dispel, None | Some(Level::Basic)) => 5,
            (Spell::Dispel, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::ProtectWater, None | Some(Level::Basic)) => 5,
            (Spell::ProtectWater, Some(Level::Advanced | Level::Expert)) => 4,

            (Spell::RemoveObstacle, None | Some(Level::Basic)) => 7,
            (Spell::RemoveObstacle, Some(Level::Advanced | Level::Expert)) => 5,

            (Spell::Weakness, None | Some(Level::Basic)) => 8,
            (Spell::Weakness, Some(Level::Advanced | Level::Expert)) => 6,

            (Spell::Forgetfulness, None | Some(Level::Basic)) => 12,
            (Spell::Forgetfulness, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::Mirth, None | Some(Level::Basic)) => 12,
            (Spell::Mirth, Some(Level::Advanced | Level::Expert)) => 9,

            (Spell::Teleport, None) => 15,
            (Spell::Teleport, Some(Level::Basic)) => 12,
            (Spell::Teleport, Some(Level::Advanced)) => 6,
            (Spell::Teleport, Some(Level::Expert)) => 3,

            (Spell::Clone, None | Some(Level::Basic)) => 24,
            (Spell::Clone, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Prayer, None | Some(Level::Basic)) => 16,
            (Spell::Prayer, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::SummonWater, None | Some(Level::Basic)) => 25,
            (Spell::SummonWater, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Visions, None | Some(Level::Basic)) => 4,
            (Spell::Visions, Some(Level::Advanced | Level::Expert)) => 2,

            (Spell::ViewAir, None | Some(Level::Basic)) => 2,
            (Spell::ViewAir, Some(Level::Advanced | Level::Expert)) => 1,

            (Spell::Disguise, None | Some(Level::Basic)) => 4,
            (Spell::Disguise, Some(Level::Advanced | Level::Expert)) => 2,

            (Spell::DimensionDoor, None | Some(Level::Basic)) => 25,
            (Spell::DimensionDoor, Some(Level::Advanced | Level::Expert)) => 20,

            (Spell::Fly, None | Some(Level::Basic)) => 20,
            (Spell::Fly, Some(Level::Advanced | Level::Expert)) => 15,

            (Spell::ViewEarth, None | Some(Level::Basic)) => 2,
            (Spell::ViewEarth, Some(Level::Advanced | Level::Expert)) => 1,

            (Spell::TownPortal, None | Some(Level::Basic)) => 16,
            (Spell::TownPortal, Some(Level::Advanced | Level::Expert)) => 12,

            (Spell::SummonBoat, None | Some(Level::Basic)) => 8,
            (Spell::SummonBoat, Some(Level::Advanced | Level::Expert)) => 7,

            (Spell::ScuttleBoat, None | Some(Level::Basic)) => 8,
            (Spell::ScuttleBoat, Some(Level::Advanced | Level::Expert)) => 6,

            (Spell::WaterWalk, None | Some(Level::Basic)) => 12,
            (Spell::WaterWalk, Some(Level::Advanced | Level::Expert)) => 8,
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
