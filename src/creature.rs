pub struct CreatureStats {
    attack: u8,
    defence: u8,
    damage: (u8, u8),
    health: u16,
    speed: u8,
    ammo_capacity: u8
}

#[derive(PartialEq)]
pub enum Town {
    Castle,
    Rampart,
    Tower,
    Inferno,
    Necropolis,
    Dungeon,
    Stronghold,
    Fortress,
    Conflux,
    // Не города, но пускай тоже будут
    Neutral,
    WarMachines
}

#[derive(PartialEq, PartialOrd)]
pub enum Creature {
    // Castle
    Pikeman,
    Halberdier,
    Archer,
    Marksman,
    Griffin,
    RoyalGriffin,
    Swordsman,
    Crusader,
    Monk,
    Zealot,
    Cavalier,
    Champion,
    Angel,
    Archangel,
    // Rampart
    Centaur,
    CentaurCaptain,
    Dwarf,
    BattleDwarf,
    WoodElf,
    GrandElf,
    Pegasus,
    SilverPegasus,
    DendroidGuard,
    DendroidSoldier,
    Unicorn,
    WarUnicorn,
    GreenDragon,
    GoldDragon,
    // Tower,
    Gremlin,
    MasterGremlin,
    StoneGargoyle,
    ObsidianGargoyle,
    StoneGolem,
    IronGolem,
    Mage,
    ArchMage,
    Genie,
    MasterGenie,
    Naga,
    NagaQueen,
    Giant,
    Titan,
    // Inferno
    Imp,
    Familiar,
    Gog,
    Magog,
    HellHound,
    Cerberus,
    Demon,
    HornedDemon,
    PitFiend,
    PitLord,
    Efreeti,
    EfreetSultan,
    Devil,
    ArchDevil,
    // Necropolis
    Skeleton,
    SkeletonWarrior,
    WalkingDead,
    Zombie,
    Wight,
    Wraith,
    Vampire,
    VampireLord,
    Lich,
    PowerLich,
    BlackKnight,
    DreadKnight,
    BoneDragon,
    GhostDragon,
    // Dungeon
    Troglodyte,
    InfernalTroglodyte,
    Harpy,
    HarpyHag,
    Beholder,
    EvilEye,
    Medusa,
    MedusaQueen,
    Minotaur,
    MinotaurKing,
    Manticore,
    Scorpicore,
    RedDragon,
    BlackDragon,
    // Stronghold,
    Goblin,
    Hobgoblin,
    WolfRider,
    WolfRaider,
    Orc,
    OrcChieftain,
    Ogre,
    OgreMagi,
    Roc,
    Thunderbird,
    Cyclops,
    CyclopsKing,
    Behemoth,
    AncientBehemoth,
    // Fortress,
    Gnoll,
    GnollMarauder,
    Lizardman,
    LizardWarrior,
    SerpentFly,
    DragonFly,
    Basilisk,
    GreaterBasilisk,
    Gorgon,
    MightyGorgon,
    Wyvern,
    WyvernMonarch,
    Hydra,
    ChaosHydra,
    // Conflux
    Pixie,
    Sprite,
    AirElemental,
    StormElemental,
    WaterElemental,
    IceElemental,
    FireElemental,
    EnergyElemental,
    EarthElemental,
    MagmaElemental,
    PsychicElemental,
    MagicElemental,
    Firebird,
    Phoenix,
    // Neutral
    Peasant,
    Halfling,
    Boar,
    Rogue,
    Mummy,
    Nomad,
    Sharpshooter,
    Troll,
    GoldGolem,
    DiamondGolem,
    Enchanter,
    FaerieDragon,
    RustDragon,
    CrystalDragon,
    AzureDragon,
    // War Machines
    Ballista,
    FirstAidTent,
    Catapult,
    AmmoCart
}

// Поменьше текста
type C  = Creature;
type CS = CreatureStats;

impl Creature {
    const fn base_stats(&self) -> CreatureStats {
        match self {
            // Castle
            Self::Pikeman => CS {
                attack: 4,
                defence: 5,
                damage: (1, 3),
                health: 10,
                speed: 4,
                ammo_capacity: 0
            },
            Self::Halberdier => CS {
                attack: 6,
                defence: 5,
                damage: (2, 3),
                health: 10,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Archer => CS {
                attack: 6,
                defence: 3,
                damage: (2, 3),
                health: 10,
                speed: 4,
                ammo_capacity: 12
            },
            Self::Marksman => CS {
                attack: 6,
                defence: 3,
                damage: (2, 3),
                health: 10,
                speed: 6,
                ammo_capacity: 24
            },
            Self::Griffin => CS {
                attack: 8,
                defence: 8,
                damage: (3, 6),
                health: 25,
                speed: 6,
                ammo_capacity: 0
            },
            Self::RoyalGriffin => CS {
                attack: 9,
                defence: 9,
                damage: (3, 6),
                health: 25,
                speed: 9,
                ammo_capacity: 0
            },
            Self::Swordsman => CS {
                attack: 10,
                defence: 12,
                damage: (6, 9),
                health: 35,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Crusader => CS {
                attack: 12,
                defence: 12,
                damage: (7, 10),
                health: 35,
                speed: 6,
                ammo_capacity: 0
            },
            Self::Monk => CS {
                attack: 12,
                defence: 7,
                damage: (10, 12),
                health: 30,
                speed: 5,
                ammo_capacity: 12
            },
            Self::Zealot => CS {
                attack: 12,
                defence: 10,
                damage: (10, 12),
                health: 30,
                speed: 7,
                ammo_capacity: 24
            },
            Self::Cavalier => CS {
                attack: 15,
                defence: 15,
                damage: (15, 25),
                health: 100,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Champion => CS {
                attack: 16,
                defence: 16,
                damage: (20, 25),
                health: 100,
                speed: 9,
                ammo_capacity: 0
            },
            Self::Angel => CS {
                attack: 20,
                defence: 20,
                damage: (50, 50),
                health: 200,
                speed: 12,
                ammo_capacity: 0
            },
            Self::Archangel => CS {
                attack: 30,
                defence: 30,
                damage: (50, 50),
                health: 250,
                speed: 18,
                ammo_capacity: 0
            },
            // Rampart
            Self::Centaur => CS {
                attack: 5,
                defence: 3,
                damage: (2, 3),
                health: 8,
                speed: 6,
                ammo_capacity: 0
            },
            Self::CentaurCaptain => CS {
                attack: 6,
                defence: 3,
                damage: (2, 3),
                health: 10,
                speed: 8,
                ammo_capacity: 0
            },
            Self::Dwarf => CS {
                attack: 6,
                defence: 7,
                damage: (2, 4),
                health: 20,
                speed: 3,
                ammo_capacity: 0
            },
            Self::BattleDwarf => CS {
                attack: 7,
                defence: 7,
                damage: (2, 4),
                health: 20,
                speed: 5,
                ammo_capacity: 0
            },
            Self::WoodElf => CS {
                attack: 9,
                defence: 5,
                damage: (3, 5),
                health: 15,
                speed: 6,
                ammo_capacity: 24
            },
            Self::GrandElf => CS {
                attack: 9,
                defence: 5,
                damage: (3, 5),
                health: 15,
                speed: 7,
                ammo_capacity: 24
            },
            Self::Pegasus => CS {
                attack: 9,
                defence: 8,
                damage: (5, 9),
                health: 30,
                speed: 8,
                ammo_capacity: 0
            },
            Self::SilverPegasus => CS {
                attack: 9,
                defence: 10,
                damage: (5, 9),
                health: 30,
                speed: 12,
                ammo_capacity: 0
            },
            Self::DendroidGuard => CS {
                attack: 9,
                defence: 12,
                damage: (10, 14),
                health: 55,
                speed: 3,
                ammo_capacity: 0
            },
            Self::DendroidSoldier => CS {
                attack: 9,
                defence: 12,
                damage: (10, 14),
                health: 65,
                speed: 4,
                ammo_capacity: 0
            },
            Self::Unicorn => CS {
                attack: 15,
                defence: 14,
                damage: (18, 22),
                health: 90,
                speed: 7,
                ammo_capacity: 0
            },
            Self::WarUnicorn => CS {
                attack: 15,
                defence: 14,
                damage: (18, 22),
                health: 110,
                speed: 9,
                ammo_capacity: 0
            },
            Self::GreenDragon => CS {
                attack: 18,
                defence: 18,
                damage: (40, 50),
                health: 180,
                speed: 10,
                ammo_capacity: 0
            },
            Self::GoldDragon => CS {
                attack: 27,
                defence: 27,
                damage: (40, 50),
                health: 250,
                speed: 16,
                ammo_capacity: 0
            },
            // Tower
            Self::Gremlin => CS {
                attack: 3,
                defence: 3,
                damage: (1, 2),
                health: 4,
                speed: 4,
                ammo_capacity: 0
            },
            Self::MasterGremlin => CS {
                attack: 4,
                defence: 4,
                damage: (1, 2),
                health: 4,
                speed: 5,
                ammo_capacity: 8
            },
            Self::StoneGargoyle => CS {
                attack: 6,
                defence: 6,
                damage: (2, 3),
                health: 16,
                speed: 6,
                ammo_capacity: 0
            },
            Self::ObsidianGargoyle => CS {
                attack: 7,
                defence: 7,
                damage: (2, 3),
                health: 16,
                speed: 9,
                ammo_capacity: 0
            },
            Self::StoneGolem => CS {
                attack: 7,
                defence: 10,
                damage: (4, 5),
                health: 30,
                speed: 3,
                ammo_capacity: 0
            },
            Self::IronGolem => CS {
                attack: 9,
                defence: 10,
                damage: (4, 5),
                health: 35,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Mage => CS {
                attack: 11,
                defence: 8,
                damage: (7, 9),
                health: 25,
                speed: 5,
                ammo_capacity: 24
            },
            Self::ArchMage => CS {
                attack: 12,
                defence: 9,
                damage: (7, 9),
                health: 30,
                speed: 7,
                ammo_capacity: 24
            },
            Self::Genie => CS {
                attack: 12,
                defence: 12,
                damage: (13, 16),
                health: 40,
                speed: 7,
                ammo_capacity: 0
            },
            Self::MasterGenie => CS {
                attack: 12,
                defence: 12,
                damage: (13, 16),
                health: 40,
                speed: 11,
                ammo_capacity: 0
            },
            Self::Naga => CS {
                attack: 16,
                defence: 13,
                damage: (20, 20),
                health: 110,
                speed: 5,
                ammo_capacity: 0
            },
            Self::NagaQueen => CS {
                attack: 16,
                defence: 13,
                damage: (30, 30),
                health: 110,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Giant => CS {
                attack: 19,
                defence: 16,
                damage: (40, 60),
                health: 150,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Titan => CS {
                attack: 24,
                defence: 24,
                damage: (40, 60),
                health: 300,
                speed: 11,
                ammo_capacity: 24
            },
            // Inferno
            Self::Imp => CS {
                attack: 2,
                defence: 3,
                damage: (1, 2),
                health: 4,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Familiar => CS {
                attack: 4,
                defence: 4,
                damage: (1, 2),
                health: 4,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Gog => CS {
                attack: 6,
                defence: 4,
                damage: (2, 4),
                health: 13,
                speed: 4,
                ammo_capacity: 12
            },
            Self::Magog => CS {
                attack: 7,
                defence: 4,
                damage: (2, 4),
                health: 13,
                speed: 6,
                ammo_capacity: 24
            },
            Self::HellHound => CS {
                attack: 10,
                defence: 6,
                damage: (2, 7),
                health: 25,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Cerberus => CS {
                attack: 10,
                defence: 8,
                damage: (2, 7),
                health: 25,
                speed: 8,
                ammo_capacity: 0
            },
            Self::Demon => CS {
                attack: 10,
                defence: 10,
                damage: (7, 9),
                health: 35,
                speed: 5,
                ammo_capacity: 0
            },
            Self::HornedDemon => CS {
                attack: 10,
                defence: 10,
                damage: (7, 9),
                health: 40,
                speed: 6,
                ammo_capacity: 0
            },
            Self::PitFiend => CS {
                attack: 13,
                defence: 13,
                damage: (13, 17),
                health: 45,
                speed: 6,
                ammo_capacity: 0
            },
            Self::PitLord => CS {
                attack: 13,
                defence: 13,
                damage: (13, 17),
                health: 45,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Efreeti => CS {
                attack: 16,
                defence: 12,
                damage: (16, 24),
                health: 90,
                speed: 9,
                ammo_capacity: 0
            },
            Self::EfreetSultan => CS {
                attack: 16,
                defence: 14,
                damage: (16, 24),
                health: 90,
                speed: 13,
                ammo_capacity: 0
            },
            Self::Devil => CS {
                attack: 19,
                defence: 21,
                damage: (30, 40),
                health: 160,
                speed: 11,
                ammo_capacity: 0
            },
            Self::ArchDevil => CS {
                attack: 26,
                defence: 28,
                damage: (30, 40),
                health: 200,
                speed: 17,
                ammo_capacity: 0
            },
            // Necropolis
            Self::Skeleton => CS {
                attack: 5,
                defence: 4,
                damage: (1, 3),
                health: 6,
                speed: 4,
                ammo_capacity: 0
            },
            Self::SkeletonWarrior => CS {
                attack: 6,
                defence: 6,
                damage: (1, 3),
                health: 6,
                speed: 5,
                ammo_capacity: 0
            },
            Self::WalkingDead => CS {
                attack: 5,
                defence: 5,
                damage: (2, 3),
                health: 15,
                speed: 3,
                ammo_capacity: 0
            },
            Self::Zombie => CS {
                attack: 5,
                defence: 5,
                damage: (2, 3),
                health: 20,
                speed: 4,
                ammo_capacity: 0
            },
            Self::Wight => CS {
                attack: 7,
                defence: 7,
                damage: (3, 5),
                health: 18,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Wraith => CS {
                attack: 7,
                defence: 7,
                damage: (3, 5),
                health: 18,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Vampire => CS {
                attack: 10,
                defence: 9,
                damage: (5, 8),
                health: 30,
                speed: 6,
                ammo_capacity: 0
            },
            Self::VampireLord => CS {
                attack: 10,
                defence: 10,
                damage: (5, 8),
                health: 40,
                speed: 9,
                ammo_capacity: 0
            },
            Self::Lich => CS {
                attack: 13,
                defence: 10,
                damage: (11, 13),
                health: 30,
                speed: 6,
                ammo_capacity: 12
            },
            Self::PowerLich => CS {
                attack: 13,
                defence: 10,
                damage: (11, 15),
                health: 40,
                speed: 7,
                ammo_capacity: 24
            },
            Self::BlackKnight => CS {
                attack: 16,
                defence: 16,
                damage: (15, 30),
                health: 120,
                speed: 7,
                ammo_capacity: 0
            },
            Self::DreadKnight => CS {
                attack: 18,
                defence: 18,
                damage: (15, 30),
                health: 120,
                speed: 9,
                ammo_capacity: 0
            },
            Self::BoneDragon => CS {
                attack: 17,
                defence: 15,
                damage: (25, 50),
                health: 150,
                speed: 9,
                ammo_capacity: 0
            },
            Self::GhostDragon => CS {
                attack: 19,
                defence: 17,
                damage: (25, 50),
                health: 200,
                speed: 14,
                ammo_capacity: 0
            },
            // Dungeon
            Self::Troglodyte => CS {
                attack: 4,
                defence: 3,
                damage: (1, 3),
                health: 5,
                speed: 4,
                ammo_capacity: 0
            },
            Self::InfernalTroglodyte => CS {
                attack: 5,
                defence: 4,
                damage: (1, 3),
                health: 6,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Harpy => CS {
                attack: 6,
                defence: 5,
                damage: (1, 4),
                health: 14,
                speed: 6,
                ammo_capacity: 0
            },
            Self::HarpyHag => CS {
                attack: 6,
                defence: 6,
                damage: (1, 4),
                health: 14,
                speed: 9,
                ammo_capacity: 0
            },
            Self::Beholder => CS {
                attack: 9,
                defence: 7,
                damage: (3, 5),
                health: 22,
                speed: 5,
                ammo_capacity: 12
            },
            Self::EvilEye => CS {
                attack: 10,
                defence: 8,
                damage: (3, 5),
                health: 22,
                speed: 7,
                ammo_capacity: 24
            },
            Self::Medusa => CS {
                attack: 9,
                defence: 9,
                damage: (6, 8),
                health: 25,
                speed: 5,
                ammo_capacity: 4
            },
            Self::MedusaQueen => CS {
                attack: 10,
                defence: 10,
                damage: (6, 8),
                health: 30,
                speed: 6,
                ammo_capacity: 8
            },
            Self::Minotaur => CS {
                attack: 14,
                defence: 12,
                damage: (12, 20),
                health: 50,
                speed: 6,
                ammo_capacity: 0
            },
            Self::MinotaurKing => CS {
                attack: 15,
                defence: 15,
                damage: (12, 20),
                health: 50,
                speed: 8,
                ammo_capacity: 0
            },
            Self::Manticore => CS {
                attack: 15,
                defence: 13,
                damage: (14, 20),
                health: 80,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Scorpicore => CS {
                attack: 16,
                defence: 14,
                damage: (14, 20),
                health: 80,
                speed: 11,
                ammo_capacity: 0
            },
            Self::RedDragon => CS {
                attack: 19,
                defence: 19,
                damage: (40, 50),
                health: 180,
                speed: 11,
                ammo_capacity: 0
            },
            Self::BlackDragon => CS {
                attack: 25,
                defence: 25,
                damage: (40, 50),
                health: 300,
                speed: 15,
                ammo_capacity: 0
            },
            // Stronghold
            Self::Goblin => CS {
                attack: 4,
                defence: 2,
                damage: (1, 2),
                health: 5,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Hobgoblin => CS {
                attack: 5,
                defence: 3,
                damage: (1, 2),
                health: 5,
                speed: 7,
                ammo_capacity: 0
            },
            Self::WolfRider => CS {
                attack: 7,
                defence: 5,
                damage: (2, 4),
                health: 10,
                speed: 6,
                ammo_capacity: 0
            },
            Self::WolfRaider => CS {
                attack: 8,
                defence: 5,
                damage: (3, 4),
                health: 10,
                speed: 8,
                ammo_capacity: 0
            },
            Self::Orc => CS {
                attack: 8,
                defence: 4,
                damage: (2, 5),
                health: 15,
                speed: 4,
                ammo_capacity: 12
            },
            Self::OrcChieftain => CS {
                attack: 8,
                defence: 4,
                damage: (2, 5),
                health: 20,
                speed: 5,
                ammo_capacity: 24
            },
            Self::Ogre => CS {
                attack: 13,
                defence: 7,
                damage: (6, 12),
                health: 40,
                speed: 4,
                ammo_capacity: 0
            },
            Self::OgreMagi => CS {
                attack: 13,
                defence: 7,
                damage: (6, 12),
                health: 60,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Roc => CS {
                attack: 13,
                defence: 11,
                damage: (11, 15),
                health: 60,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Thunderbird => CS {
                attack: 13,
                defence: 11,
                damage: (11, 15),
                health: 60,
                speed: 11,
                ammo_capacity: 0
            },
            Self::Cyclops => CS {
                attack: 15,
                defence: 12,
                damage: (16, 20),
                health: 70,
                speed: 6,
                ammo_capacity: 16
            },
            Self::CyclopsKing => CS {
                attack: 17,
                defence: 13,
                damage: (16, 20),
                health: 70,
                speed: 8,
                ammo_capacity: 24
            },
            Self::Behemoth => CS {
                attack: 17,
                defence: 17,
                damage: (30, 50),
                health: 160,
                speed: 6,
                ammo_capacity: 0
            },
            Self::AncientBehemoth => CS {
                attack: 19,
                defence: 19,
                damage: (30, 50),
                health: 300,
                speed: 9,
                ammo_capacity: 0
            },
            // Fortress
            Self::Gnoll => CS {
                attack: 3,
                defence: 5,
                damage: (2, 3),
                health: 6,
                speed: 4,
                ammo_capacity: 0
            },
            Self::GnollMarauder => CS {
                attack: 4,
                defence: 6,
                damage: (2, 3),
                health: 6,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Lizardman => CS {
                attack: 5,
                defence: 6,
                damage: (2, 3),
                health: 14,
                speed: 4,
                ammo_capacity: 12
            },
            Self::LizardWarrior => CS {
                attack: 6,
                defence: 8,
                damage: (2, 5),
                health: 15,
                speed: 5,
                ammo_capacity: 24
            },
            Self::SerpentFly => CS {
                attack: 7,
                defence: 9,
                damage: (2, 5),
                health: 20,
                speed: 9,
                ammo_capacity: 0
            },
            Self::DragonFly => CS {
                attack: 8,
                defence: 10,
                damage: (2, 5),
                health: 20,
                speed: 13,
                ammo_capacity: 0
            },
            Self::Basilisk => CS {
                attack: 11,
                defence: 11,
                damage: (6, 10),
                health: 35,
                speed: 5,
                ammo_capacity: 0
            },
            Self::GreaterBasilisk => CS {
                attack: 12,
                defence: 12,
                damage: (6, 10),
                health: 40,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Gorgon => CS {
                attack: 10,
                defence: 14,
                damage: (12, 16),
                health: 70,
                speed: 5,
                ammo_capacity: 0
            },
            Self::MightyGorgon => CS {
                attack: 11,
                defence: 16,
                damage: (12, 16),
                health: 70,
                speed: 6,
                ammo_capacity: 0
            },
            Self::Wyvern => CS {
                attack: 14,
                defence: 14,
                damage: (14, 18),
                health: 70,
                speed: 7,
                ammo_capacity: 0
            },
            Self::WyvernMonarch => CS {
                attack: 14,
                defence: 14,
                damage: (18, 22),
                health: 70,
                speed: 11,
                ammo_capacity: 0
            },
            Self::Hydra => CS {
                attack: 16,
                defence: 18,
                damage: (25, 45),
                health: 175,
                speed: 5,
                ammo_capacity: 0
            },
            Self::ChaosHydra => CS {
                attack: 18,
                defence: 20,
                damage: (25, 45),
                health: 250,
                speed: 7,
                ammo_capacity: 0
            },
            // Conflux
            Self::Pixie => CS {
                attack: 2,
                defence: 2,
                damage: (1, 2),
                health: 3,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Sprite => CS {
                attack: 2,
                defence: 2,
                damage: (1, 3),
                health: 3,
                speed: 9,
                ammo_capacity: 0
            },
            Self::AirElemental => CS {
                attack: 9,
                defence: 9,
                damage: (2, 8),
                health: 25,
                speed: 7,
                ammo_capacity: 0
            },
            Self::StormElemental => CS {
                attack: 9,
                defence: 9,
                damage: (2, 8),
                health: 25,
                speed: 8,
                ammo_capacity: 24
            },
            Self::WaterElemental => CS {
                attack: 8,
                defence: 10,
                damage: (3, 7),
                health: 30,
                speed: 5,
                ammo_capacity: 0
            },
            Self::IceElemental => CS {
                attack: 8,
                defence: 10,
                damage: (3, 7),
                health: 30,
                speed: 6,
                ammo_capacity: 24
            },
            Self::FireElemental => CS {
                attack: 10,
                defence: 8,
                damage: (4, 6),
                health: 35,
                speed: 6,
                ammo_capacity: 0
            },
            Self::EnergyElemental => CS {
                attack: 12,
                defence: 8,
                damage: (4, 6),
                health: 35,
                speed: 8,
                ammo_capacity: 0
            },
            Self::EarthElemental => CS {
                attack: 10,
                defence: 10,
                damage: (4, 8),
                health: 40,
                speed: 4,
                ammo_capacity: 0
            },
            Self::MagmaElemental => CS {
                attack: 11,
                defence: 11,
                damage: (6, 10),
                health: 40,
                speed: 6,
                ammo_capacity: 0
            },
            Self::PsychicElemental => CS {
                attack: 15,
                defence: 13,
                damage: (10, 20),
                health: 75,
                speed: 7,
                ammo_capacity: 0
            },
            Self::MagicElemental => CS {
                attack: 15,
                defence: 13,
                damage: (15, 25),
                health: 80,
                speed: 9,
                ammo_capacity: 0
            },
            Self::Firebird => CS {
                attack: 18,
                defence: 18,
                damage: (30, 40),
                health: 150,
                speed: 15,
                ammo_capacity: 0
            },
            Self::Phoenix => CS {
                attack: 21,
                defence: 18,
                damage: (30, 40),
                health: 200,
                speed: 21,
                ammo_capacity: 0
            },
            // Neutral
            Self::Peasant => CS {
                attack: 1,
                defence: 1,
                damage: (1, 1),
                health: 1,
                speed: 3,
                ammo_capacity: 0
            },
            Self::Halfling => CS {
                attack: 4,
                defence: 2,
                damage: (1, 3),
                health: 4,
                speed: 5,
                ammo_capacity: 24
            },
            Self::Boar => CS {
                attack: 6,
                defence: 5,
                damage: (2, 3),
                health: 15,
                speed: 6,
                ammo_capacity: 0
            },
            Self::Rogue => CS {
                attack: 8,
                defence: 3,
                damage: (2, 4),
                health: 10,
                speed: 6,
                ammo_capacity: 0
            },
            Self::Mummy => CS {
                attack: 7,
                defence: 7,
                damage: (3, 5),
                health: 30,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Nomad => CS {
                attack: 9,
                defence: 8,
                damage: (2, 6),
                health: 30,
                speed: 7,
                ammo_capacity: 0
            },
            Self::Sharpshooter => CS {
                attack: 12,
                defence: 10,
                damage: (8, 10),
                health: 15,
                speed: 9,
                ammo_capacity: 32
            },
            Self::Troll => CS {
                attack: 14,
                defence: 7,
                damage: (10, 15),
                health: 40,
                speed: 7,
                ammo_capacity: 0
            },
            Self::GoldGolem => CS {
                attack: 11,
                defence: 12,
                damage: (8, 10),
                health: 50,
                speed: 5,
                ammo_capacity: 0
            },
            Self::DiamondGolem => CS {
                attack: 13,
                defence: 12,
                damage: (10, 14),
                health: 60,
                speed: 5,
                ammo_capacity: 0
            },
            Self::Enchanter => CS {
                attack: 17,
                defence: 12,
                damage: (14, 14),
                health: 30,
                speed: 9,
                ammo_capacity: 32
            },
            Self::FaerieDragon => CS {
                attack: 20,
                defence: 20,
                damage: (20, 30),
                health: 500,
                speed: 15,
                ammo_capacity: 0
            },
            Self::RustDragon => CS {
                attack: 30,
                defence: 30,
                damage: (50, 50),
                health: 750,
                speed: 17,
                ammo_capacity: 0
            },
            Self::CrystalDragon => CS {
                attack: 40,
                defence: 40,
                damage: (60, 75),
                health: 800,
                speed: 16,
                ammo_capacity: 0
            },
            Self::AzureDragon => CS {
                attack: 50,
                defence: 50,
                damage: (70, 80),
                health: 1000,
                speed: 19,
                ammo_capacity: 0
            },
            // War Machines
            Self::Ballista => CS {
                attack: 10,
                defence: 10,
                damage: (2, 3),
                health: 250,
                speed: 0,
                ammo_capacity: 0
            },
            Self::FirstAidTent => CS {
                attack: 0,
                defence: 0,
                damage: (0, 0),
                health: 75,
                speed: 0,
                ammo_capacity: 0
            },
            Self::Catapult => CS {
                attack: 10,
                defence: 10,
                damage: (0, 0),
                health: 1000,
                speed: 0,
                ammo_capacity: 0
            },
            Self::AmmoCart => CS {
                attack: 0,
                defence: 5,
                damage: (0, 0),
                health: 100,
                speed: 0,
                ammo_capacity: 0
            }
        }
    }

    fn town(&self) -> Town {
        match self {
            x if (Self::Pikeman..=Self::Archangel).contains(x) => Town::Castle,
            x if (Self::Centaur..=Self::GoldDragon).contains(x) => Town::Rampart,
            x if (Self::Gremlin..=Self::Titan).contains(x) => Town::Tower,
            x if (Self::Imp..=Self::ArchDevil).contains(x) => Town::Inferno,
            x if (Self::Skeleton..=Self::GhostDragon).contains(x) => Town::Necropolis,
            x if (Self::Troglodyte..=Self::BlackDragon).contains(x) => Town::Dungeon,
            x if (Self::Goblin..=Self::AncientBehemoth).contains(x) => Town::Stronghold,
            x if (Self::Gnoll..=Self::ChaosHydra).contains(x) => Town::Fortress,
            x if (Self::Pixie..=Self::Phoenix).contains(x) => Town::Conflux,
            x if (Self::Peasant..=Self::AzureDragon).contains(x) => Town::Neutral,
            x if (Self::Ballista..=Self::AmmoCart).contains(x) => Town::WarMachines,
            _ => panic!("Creature without town!")
        }
    }

    fn is_wide(&self) -> bool {
        const WIDE_CREATURES: [Creature; 53] = [
            // Castle
            C::Griffin, C::RoyalGriffin,
            C::Cavalier, C::Champion,
            C::Archangel,
            // Rampart
            C::Centaur, C::CentaurCaptain,
            C::Pegasus, C::SilverPegasus,
            C::Unicorn, C::WarUnicorn,
            C::GreenDragon, C::GoldDragon,
            // Tower
            C::Naga, C::NagaQueen,
            // Inferno
            C::HellHound, C::Cerberus,
            // Necropolis
            C::BlackKnight, C::DreadKnight,
            C::BoneDragon, C::GhostDragon,
            // Dungeon
            C::Medusa, C::MedusaQueen,
            C::Manticore, C::Scorpicore,
            C::RedDragon, C::BlackDragon,
            // Stronghold
            C::WolfRider, C::WolfRaider,
            C::Roc, C::Thunderbird,
            C::Behemoth, C::AncientBehemoth,
            // Fortress
            C::Basilisk, C::GreaterBasilisk,
            C::Gorgon, C::MightyGorgon,
            C::Wyvern, C::WyvernMonarch,
            C::Hydra, C::ChaosHydra,
            // Conflux
            C::WaterElemental, C::IceElemental,
            C::Firebird, C::Phoenix,
            // Neutral
            C::Boar, C::Nomad,
            C::FaerieDragon, C::RustDragon, C::CrystalDragon, C::AzureDragon,
            // War Machines
            C::Ballista, C::Catapult
        ];
        WIDE_CREATURES.contains(self)
    }

    fn is_flying(&self) -> bool {
        const FLYING_CREATURES: [Creature; 42] = [
            // Castle
            C::Griffin, C::RoyalGriffin,
            C::Angel, C::Archangel,
            // Rampart
            C::Pegasus, C::SilverPegasus,
            C::GreenDragon, C::GoldDragon,
            // Tower
            C::StoneGargoyle, C::ObsidianGargoyle,
            C::Genie, C::MasterGenie,
            // Inferno
            C::Efreeti, C::EfreetSultan,
            C::Devil, C::ArchDevil,
            // Necropolis
            C::Wight, C::Wraith,
            C::Vampire, C::VampireLord,
            C::BoneDragon, C::GhostDragon,
            // Dungeon
            C::Harpy, C::HarpyHag,
            C::Manticore, C::Scorpicore,
            C::RedDragon, C::BlackDragon,
            // Stronghold
            C::Roc, C::Thunderbird,
            // Fortress
            C::SerpentFly, C::DragonFly,
            C::Wyvern, C::WyvernMonarch,
            // Conflux
            C::Pixie, C::Sprite,
            C::EnergyElemental,
            C::Firebird, C::Phoenix,
            // Neutral
            C::FaerieDragon, C::RustDragon, C::AzureDragon
        ];
        FLYING_CREATURES.contains(self)
    }

    fn is_ranged(&self) -> bool {
        self.base_stats().ammo_capacity != 0
    }

    fn is_undead(&self) -> bool {
        const NON_NECROPOLIS_UNDEAD: [Creature; 1] = [
            C::Mummy
        ];
        self.town() == Town::Necropolis || NON_NECROPOLIS_UNDEAD.contains(self)
    }
}