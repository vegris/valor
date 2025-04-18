use macros::EnumIndex;
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

use crate::spells::Spell;
use crate::traits::{ContainerType, SpriteGroupT};

pub enum Texture {
    Button(Button, ButtonState),
    Spell(Spell),
}

#[derive(Clone, Copy, EnumCount, EnumIter, IntoStaticStr)]
pub enum Button {
    Surrender,
    Retreat,
    Settings,
    AutoBattle,
    BookOfMagic,
    Wait,
    Defend,
}

#[allow(unused)]
#[derive(Clone, Copy, EnumCount, EnumIndex)]
pub enum ButtonState {
    Base,
    Pressed,
    Disabled,
    Hovered,
}

impl From<Texture> for u64 {
    fn from(value: Texture) -> Self {
        match value {
            Texture::Button(Button::Surrender, ButtonState::Base) => 1,
            Texture::Button(Button::Surrender, ButtonState::Pressed) => 2,
            Texture::Button(Button::Surrender, ButtonState::Disabled) => 3,
            Texture::Button(Button::Surrender, ButtonState::Hovered) => 4,
            Texture::Button(Button::Retreat, ButtonState::Base) => 5,
            Texture::Button(Button::Retreat, ButtonState::Pressed) => 6,
            Texture::Button(Button::Retreat, ButtonState::Disabled) => 7,
            Texture::Button(Button::Retreat, ButtonState::Hovered) => 8,
            Texture::Button(Button::Settings, ButtonState::Base) => 9,
            Texture::Button(Button::Settings, ButtonState::Pressed) => 10,
            Texture::Button(Button::Settings, ButtonState::Disabled) => 11,
            Texture::Button(Button::Settings, ButtonState::Hovered) => 12,
            Texture::Button(Button::AutoBattle, ButtonState::Base) => 13,
            Texture::Button(Button::AutoBattle, ButtonState::Pressed) => 14,
            Texture::Button(Button::AutoBattle, ButtonState::Disabled) => 15,
            Texture::Button(Button::AutoBattle, ButtonState::Hovered) => 16,
            Texture::Button(Button::BookOfMagic, ButtonState::Base) => 17,
            Texture::Button(Button::BookOfMagic, ButtonState::Pressed) => 18,
            Texture::Button(Button::BookOfMagic, ButtonState::Disabled) => 19,
            Texture::Button(Button::BookOfMagic, ButtonState::Hovered) => 20,
            Texture::Button(Button::Wait, ButtonState::Base) => 21,
            Texture::Button(Button::Wait, ButtonState::Pressed) => 22,
            Texture::Button(Button::Wait, ButtonState::Disabled) => 23,
            Texture::Button(Button::Wait, ButtonState::Hovered) => 24,
            Texture::Button(Button::Defend, ButtonState::Base) => 25,
            Texture::Button(Button::Defend, ButtonState::Pressed) => 26,
            Texture::Button(Button::Defend, ButtonState::Disabled) => 27,
            Texture::Button(Button::Defend, ButtonState::Hovered) => 28,
            Texture::Spell(Spell::SummonBoat) => 29,
            Texture::Spell(Spell::ScuttleBoat) => 30,
            Texture::Spell(Spell::Visions) => 31,
            Texture::Spell(Spell::ViewAir) => 32,
            Texture::Spell(Spell::Disguise) => 33,
            Texture::Spell(Spell::ViewEarth) => 34,
            Texture::Spell(Spell::Fly) => 35,
            Texture::Spell(Spell::WaterWalk) => 36,
            Texture::Spell(Spell::DimensionDoor) => 37,
            Texture::Spell(Spell::TownPortal) => 38,
            Texture::Spell(Spell::Quicksand) => 39,
            Texture::Spell(Spell::LandMine) => 40,
            Texture::Spell(Spell::ForceField) => 41,
            Texture::Spell(Spell::FireWall) => 42,
            Texture::Spell(Spell::Earthquake) => 43,
            Texture::Spell(Spell::MagicArrow) => 44,
            Texture::Spell(Spell::IceBolt) => 45,
            Texture::Spell(Spell::LightningBolt) => 46,
            Texture::Spell(Spell::Implosion) => 47,
            Texture::Spell(Spell::ChainLightning) => 48,
            Texture::Spell(Spell::FrostRing) => 49,
            Texture::Spell(Spell::Fireball) => 50,
            Texture::Spell(Spell::Inferno) => 51,
            Texture::Spell(Spell::MeteorShower) => 52,
            Texture::Spell(Spell::DeathRipple) => 53,
            Texture::Spell(Spell::DestroyUndead) => 54,
            Texture::Spell(Spell::Armageddon) => 55,
            Texture::Spell(Spell::Shield) => 56,
            Texture::Spell(Spell::AirShield) => 57,
            Texture::Spell(Spell::FireShield) => 58,
            Texture::Spell(Spell::ProtectAir) => 59,
            Texture::Spell(Spell::ProtectFire) => 60,
            Texture::Spell(Spell::ProtectWater) => 61,
            Texture::Spell(Spell::ProtectEarth) => 62,
            Texture::Spell(Spell::AntiMagic) => 63,
            Texture::Spell(Spell::Dispel) => 64,
            Texture::Spell(Spell::MagicMirror) => 65,
            Texture::Spell(Spell::Cure) => 66,
            Texture::Spell(Spell::Resurrection) => 67,
            Texture::Spell(Spell::AnimateDead) => 68,
            Texture::Spell(Spell::Sacrifice) => 69,
            Texture::Spell(Spell::Bless) => 70,
            Texture::Spell(Spell::Curse) => 71,
            Texture::Spell(Spell::Bloodlust) => 72,
            Texture::Spell(Spell::Precision) => 73,
            Texture::Spell(Spell::Weakness) => 74,
            Texture::Spell(Spell::StoneSkin) => 75,
            Texture::Spell(Spell::DisruptingRay) => 76,
            Texture::Spell(Spell::Prayer) => 77,
            Texture::Spell(Spell::Sorrow) => 78,
            Texture::Spell(Spell::Fortune) => 79,
            Texture::Spell(Spell::Mirth) => 80,
            Texture::Spell(Spell::Misfortune) => 81,
            Texture::Spell(Spell::Haste) => 82,
            Texture::Spell(Spell::Slow) => 83,
            Texture::Spell(Spell::Slayer) => 84,
            Texture::Spell(Spell::Frenzy) => 85,
            Texture::Spell(Spell::TitanLightningBolt) => 86,
            Texture::Spell(Spell::Counterstrike) => 87,
            Texture::Spell(Spell::Berserk) => 88,
            Texture::Spell(Spell::Hypnotize) => 89,
            Texture::Spell(Spell::Forgetfulness) => 90,
            Texture::Spell(Spell::Blind) => 91,
            Texture::Spell(Spell::Teleport) => 92,
            Texture::Spell(Spell::RemoveObstacle) => 93,
            Texture::Spell(Spell::Clone) => 94,
            Texture::Spell(Spell::SummonEarth) => 95,
            Texture::Spell(Spell::SummonFire) => 96,
            Texture::Spell(Spell::SummonWater) => 97,
            Texture::Spell(Spell::SummonAir) => 98,
        }
    }
}

impl TryFrom<u64> for Texture {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let texture = match value {
            1 => Texture::Button(Button::Surrender, ButtonState::Base),
            2 => Texture::Button(Button::Surrender, ButtonState::Pressed),
            3 => Texture::Button(Button::Surrender, ButtonState::Disabled),
            4 => Texture::Button(Button::Surrender, ButtonState::Hovered),
            5 => Texture::Button(Button::Retreat, ButtonState::Base),
            6 => Texture::Button(Button::Retreat, ButtonState::Pressed),
            7 => Texture::Button(Button::Retreat, ButtonState::Disabled),
            8 => Texture::Button(Button::Retreat, ButtonState::Hovered),
            9 => Texture::Button(Button::Settings, ButtonState::Base),
            10 => Texture::Button(Button::Settings, ButtonState::Pressed),
            11 => Texture::Button(Button::Settings, ButtonState::Disabled),
            12 => Texture::Button(Button::Settings, ButtonState::Hovered),
            13 => Texture::Button(Button::AutoBattle, ButtonState::Base),
            14 => Texture::Button(Button::AutoBattle, ButtonState::Pressed),
            15 => Texture::Button(Button::AutoBattle, ButtonState::Disabled),
            16 => Texture::Button(Button::AutoBattle, ButtonState::Hovered),
            17 => Texture::Button(Button::BookOfMagic, ButtonState::Base),
            18 => Texture::Button(Button::BookOfMagic, ButtonState::Pressed),
            19 => Texture::Button(Button::BookOfMagic, ButtonState::Disabled),
            20 => Texture::Button(Button::BookOfMagic, ButtonState::Hovered),
            21 => Texture::Button(Button::Wait, ButtonState::Base),
            22 => Texture::Button(Button::Wait, ButtonState::Pressed),
            23 => Texture::Button(Button::Wait, ButtonState::Disabled),
            24 => Texture::Button(Button::Wait, ButtonState::Hovered),
            25 => Texture::Button(Button::Defend, ButtonState::Base),
            26 => Texture::Button(Button::Defend, ButtonState::Pressed),
            27 => Texture::Button(Button::Defend, ButtonState::Disabled),
            28 => Texture::Button(Button::Defend, ButtonState::Hovered),
            29 => Texture::Spell(Spell::SummonBoat),
            30 => Texture::Spell(Spell::ScuttleBoat),
            31 => Texture::Spell(Spell::Visions),
            32 => Texture::Spell(Spell::ViewAir),
            33 => Texture::Spell(Spell::Disguise),
            34 => Texture::Spell(Spell::ViewEarth),
            35 => Texture::Spell(Spell::Fly),
            36 => Texture::Spell(Spell::WaterWalk),
            37 => Texture::Spell(Spell::DimensionDoor),
            38 => Texture::Spell(Spell::TownPortal),
            39 => Texture::Spell(Spell::Quicksand),
            40 => Texture::Spell(Spell::LandMine),
            41 => Texture::Spell(Spell::ForceField),
            42 => Texture::Spell(Spell::FireWall),
            43 => Texture::Spell(Spell::Earthquake),
            44 => Texture::Spell(Spell::MagicArrow),
            45 => Texture::Spell(Spell::IceBolt),
            46 => Texture::Spell(Spell::LightningBolt),
            47 => Texture::Spell(Spell::Implosion),
            48 => Texture::Spell(Spell::ChainLightning),
            49 => Texture::Spell(Spell::FrostRing),
            50 => Texture::Spell(Spell::Fireball),
            51 => Texture::Spell(Spell::Inferno),
            52 => Texture::Spell(Spell::MeteorShower),
            53 => Texture::Spell(Spell::DeathRipple),
            54 => Texture::Spell(Spell::DestroyUndead),
            55 => Texture::Spell(Spell::Armageddon),
            56 => Texture::Spell(Spell::Shield),
            57 => Texture::Spell(Spell::AirShield),
            58 => Texture::Spell(Spell::FireShield),
            59 => Texture::Spell(Spell::ProtectAir),
            60 => Texture::Spell(Spell::ProtectFire),
            61 => Texture::Spell(Spell::ProtectWater),
            62 => Texture::Spell(Spell::ProtectEarth),
            63 => Texture::Spell(Spell::AntiMagic),
            64 => Texture::Spell(Spell::Dispel),
            65 => Texture::Spell(Spell::MagicMirror),
            66 => Texture::Spell(Spell::Cure),
            67 => Texture::Spell(Spell::Resurrection),
            68 => Texture::Spell(Spell::AnimateDead),
            69 => Texture::Spell(Spell::Sacrifice),
            70 => Texture::Spell(Spell::Bless),
            71 => Texture::Spell(Spell::Curse),
            72 => Texture::Spell(Spell::Bloodlust),
            73 => Texture::Spell(Spell::Precision),
            74 => Texture::Spell(Spell::Weakness),
            75 => Texture::Spell(Spell::StoneSkin),
            76 => Texture::Spell(Spell::DisruptingRay),
            77 => Texture::Spell(Spell::Prayer),
            78 => Texture::Spell(Spell::Sorrow),
            79 => Texture::Spell(Spell::Fortune),
            80 => Texture::Spell(Spell::Mirth),
            81 => Texture::Spell(Spell::Misfortune),
            82 => Texture::Spell(Spell::Haste),
            83 => Texture::Spell(Spell::Slow),
            84 => Texture::Spell(Spell::Slayer),
            85 => Texture::Spell(Spell::Frenzy),
            86 => Texture::Spell(Spell::TitanLightningBolt),
            87 => Texture::Spell(Spell::Counterstrike),
            88 => Texture::Spell(Spell::Berserk),
            89 => Texture::Spell(Spell::Hypnotize),
            90 => Texture::Spell(Spell::Forgetfulness),
            91 => Texture::Spell(Spell::Blind),
            92 => Texture::Spell(Spell::Teleport),
            93 => Texture::Spell(Spell::RemoveObstacle),
            94 => Texture::Spell(Spell::Clone),
            95 => Texture::Spell(Spell::SummonEarth),
            96 => Texture::Spell(Spell::SummonFire),
            97 => Texture::Spell(Spell::SummonWater),
            98 => Texture::Spell(Spell::SummonAir),
            _ => return Err("Invalid texture id"),
        };

        Ok(texture)
    }
}

impl Button {
    pub const fn filename(self) -> &'static str {
        match self {
            Self::Surrender => "icm001.def",
            Self::Retreat => "icm002.def",
            Self::Settings => "icm003.def",
            Self::AutoBattle => "icm004.def",
            Self::BookOfMagic => "icm005.def",
            Self::Wait => "icm006.def",
            Self::Defend => "icm007.def",
        }
    }
}

impl ContainerType for ButtonState {
    const CONTAINER_TYPE: u32 = 71;
}

impl SpriteGroupT for ButtonState {}
