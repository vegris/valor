use strum_macros::EnumCount;

#[derive(Clone, Copy, PartialEq, EnumCount)]
pub enum Spell {
    CallShip,
    DestroyShip,
    Examine,
    ViewAir,
    Disguise,
    ViewEarth,
    Flight,
    WaterWalk,
    DimensionDoor,
    TownPortal,
    QuickSand,
    MineField,
    ForceField,
    FireWall,
    Earthquake,
    MagicArrow,
    IceRay,
    LightingBolt,
    Explosion,
    ChainLighting,
    FrostRing,
    Fireball,
    Firestrike,
    Meteor,
    WaveOfDeath,
    DestroyUndead,
    Armaggedon,
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
    Ressurection,
    ReanimateDead,
    Sacrifice,
    Blessing,
    Curse,
    Bloodlust,
    Precision,
    Weakness,
    StoneSkin,
    DestructiveRay,
    Pray,
    Sorrow,
    FortifyLuck,
    Mirth,
    Misfortune,
    Speed,
    Slow,
    Slayer,
    Frenzy,
    TitanRage,
    CounterAttack,
    Berserk,
    Hypnosis,
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

impl Spell {
    pub const SPRITESHEET: &'static str = "spells.def";

    pub fn spell_type(self) -> SpellType {
        let adventure_spells = [
            Spell::CallShip,
            Spell::DestroyShip,
            Spell::Examine,
            Spell::ViewAir,
            Spell::Disguise,
            Spell::ViewEarth,
            Spell::Flight,
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
