struct CreatureStats {
    attack: u8,
    defence: u8,
    damage: (u8, u8),
    health: u16,
    speed: u8
}

#[derive(Clone, Copy)]
#[allow(unused)]
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

impl Creature {
    const fn base_stats(self) -> CreatureStats {
        match self {
            // Castle
            Self::Pikeman => CreatureStats {
                attack: 4,
                defence: 5,
                damage: (1, 3),
                health: 10,
                speed: 4
            },
            Self::Halberdier => CreatureStats {
                attack: 6,
                defence: 5,
                damage: (2, 3),
                health: 10,
                speed: 5
            },
            Self::Archer => CreatureStats {
                attack: 6,
                defence: 3,
                damage: (2, 3),
                health: 10,
                speed: 4
            },
            Self::Marksman => CreatureStats {
                attack: 6,
                defence: 3,
                damage: (2, 3),
                health: 10,
                speed: 6
            },
            Self::Griffin => CreatureStats {
                attack: 8,
                defence: 8,
                damage: (3, 6),
                health: 25,
                speed: 6
            },
            Self::RoyalGriffin => CreatureStats {
                attack: 9,
                defence: 9,
                damage: (3, 6),
                health: 25,
                speed: 9
            },
            Self::Swordsman => CreatureStats {
                attack: 10,
                defence: 12,
                damage: (6, 9),
                health: 35,
                speed: 5
            },
            Self::Crusader => CreatureStats {
                attack: 12,
                defence: 12,
                damage: (7, 10),
                health: 35,
                speed: 6
            },
            Self::Monk => CreatureStats {
                attack: 12,
                defence: 7,
                damage: (10, 12),
                health: 30,
                speed: 5
            },
            Self::Zealot => CreatureStats {
                attack: 12,
                defence: 10,
                damage: (10, 12),
                health: 30,
                speed: 7
            },
            Self::Cavalier => CreatureStats {
                attack: 15,
                defence: 15,
                damage: (15, 25),
                health: 100,
                speed: 7
            },
            Self::Champion => CreatureStats {
                attack: 16,
                defence: 16,
                damage: (20, 25),
                health: 100,
                speed: 9
            },
            Self::Angel => CreatureStats {
                attack: 20,
                defence: 20,
                damage: (50, 50),
                health: 200,
                speed: 12
            },
            Self::Archangel => CreatureStats {
                attack: 30,
                defence: 30,
                damage: (50, 50),
                health: 250,
                speed: 18
            },
            // Rampart
            Self::Centaur => CreatureStats {
                attack: 5,
                defence: 3,
                damage: (2, 3),
                health: 8,
                speed: 6
            },
            Self::CentaurCaptain => CreatureStats {
                attack: 6,
                defence: 3,
                damage: (2, 3),
                health: 10,
                speed: 8
            },
            Self::Dwarf => CreatureStats {
                attack: 6,
                defence: 7,
                damage: (2, 4),
                health: 20,
                speed: 3
            },
            Self::BattleDwarf => CreatureStats {
                attack: 7,
                defence: 7,
                damage: (2, 4),
                health: 20,
                speed: 5
            },
            Self::WoodElf => CreatureStats {
                attack: 9,
                defence: 5,
                damage: (3, 5),
                health: 15,
                speed: 6
            },
            Self::GrandElf => CreatureStats {
                attack: 9,
                defence: 5,
                damage: (3, 5),
                health: 15,
                speed: 7
            },
            Self::Pegasus => CreatureStats {
                attack: 9,
                defence: 8,
                damage: (5, 9),
                health: 30,
                speed: 8
            },
            Self::SilverPegasus => CreatureStats {
                attack: 9,
                defence: 10,
                damage: (5, 9),
                health: 30,
                speed: 12
            },
            Self::DendroidGuard => CreatureStats {
                attack: 9,
                defence: 12,
                damage: (10, 14),
                health: 55,
                speed: 3
            },
            Self::DendroidSoldier => CreatureStats {
                attack: 9,
                defence: 12,
                damage: (10, 14),
                health: 65,
                speed: 4
            },
            Self::Unicorn => CreatureStats {
                attack: 15,
                defence: 14,
                damage: (18, 22),
                health: 90,
                speed: 7
            },
            Self::WarUnicorn => CreatureStats {
                attack: 15,
                defence: 14,
                damage: (18, 22),
                health: 110,
                speed: 9
            },
            Self::GreenDragon => CreatureStats {
                attack: 18,
                defence: 18,
                damage: (40, 50),
                health: 180,
                speed: 10
            },
            Self::GoldDragon => CreatureStats {
                attack: 27,
                defence: 27,
                damage: (40, 50),
                health: 250,
                speed: 16
            },
            // Tower
            Self::Gremlin => CreatureStats {
                attack: 3,
                defence: 3,
                damage: (1, 2),
                health: 4,
                speed: 4
            },
            Self::MasterGremlin => CreatureStats {
                attack: 4,
                defence: 4,
                damage: (1, 2),
                health: 4,
                speed: 5
            },
            Self::StoneGargoyle => CreatureStats {
                attack: 6,
                defence: 6,
                damage: (2, 3),
                health: 16,
                speed: 6
            },
            Self::ObsidianGargoyle => CreatureStats {
                attack: 7,
                defence: 7,
                damage: (2, 3),
                health: 16,
                speed: 9
            },
            Self::StoneGolem => CreatureStats {
                attack: 7,
                defence: 10,
                damage: (4, 5),
                health: 30,
                speed: 3
            },
            Self::IronGolem => CreatureStats {
                attack: 9,
                defence: 10,
                damage: (4, 5),
                health: 35,
                speed: 5
            },
            Self::Mage => CreatureStats {
                attack: 11,
                defence: 8,
                damage: (7, 9),
                health: 25,
                speed: 5
            },
            Self::ArchMage => CreatureStats {
                attack: 12,
                defence: 9,
                damage: (7, 9),
                health: 30,
                speed: 7
            },
            Self::Genie => CreatureStats {
                attack: 12,
                defence: 12,
                damage: (13, 16),
                health: 40,
                speed: 7
            },
            Self::MasterGenie => CreatureStats {
                attack: 12,
                defence: 12,
                damage: (13, 16),
                health: 40,
                speed: 11
            },
            Self::Naga => CreatureStats {
                attack: 16,
                defence: 13,
                damage: (20, 20),
                health: 110,
                speed: 5
            },
            Self::NagaQueen => CreatureStats {
                attack: 16,
                defence: 13,
                damage: (30, 30),
                health: 110,
                speed: 7
            },
            Self::Giant => CreatureStats {
                attack: 19,
                defence: 16,
                damage: (40, 60),
                health: 150,
                speed: 7
            },
            Self::Titan => CreatureStats {
                attack: 24,
                defence: 24,
                damage: (40, 60),
                health: 300,
                speed: 11
            },
            // Inferno
            Self::Imp => CreatureStats {
                attack: 2,
                defence: 3,
                damage: (1, 2),
                health: 4,
                speed: 5
            },
            Self::Familiar => CreatureStats {
                attack: 4,
                defence: 4,
                damage: (1, 2),
                health: 4,
                speed: 7
            },
            Self::Gog => CreatureStats {
                attack: 6,
                defence: 4,
                damage: (2, 4),
                health: 13,
                speed: 4
            },
            Self::Magog => CreatureStats {
                attack: 7,
                defence: 4,
                damage: (2, 4),
                health: 13,
                speed: 6
            },
            Self::HellHound => CreatureStats {
                attack: 10,
                defence: 6,
                damage: (2, 7),
                health: 25,
                speed: 7
            },
            Self::Cerberus => CreatureStats {
                attack: 10,
                defence: 8,
                damage: (2, 7),
                health: 25,
                speed: 8
            },
            Self::Demon => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (7, 9),
                health: 35,
                speed: 5
            },
            Self::HornedDemon => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (7, 9),
                health: 40,
                speed: 6
            },
            Self::PitFiend => CreatureStats {
                attack: 13,
                defence: 13,
                damage: (13, 17),
                health: 45,
                speed: 6
            },
            Self::PitLord => CreatureStats {
                attack: 13,
                defence: 13,
                damage: (13, 17),
                health: 45,
                speed: 7
            },
            Self::Efreeti => CreatureStats {
                attack: 16,
                defence: 12,
                damage: (16, 24),
                health: 90,
                speed: 9
            },
            Self::EfreetSultan => CreatureStats {
                attack: 16,
                defence: 14,
                damage: (16, 24),
                health: 90,
                speed: 13
            },
            Self::Devil => CreatureStats {
                attack: 19,
                defence: 21,
                damage: (30, 40),
                health: 160,
                speed: 11
            },
            Self::ArchDevil => CreatureStats {
                attack: 26,
                defence: 28,
                damage: (30, 40),
                health: 200,
                speed: 17
            },
            // Necropolis
            Self::Skeleton => CreatureStats {
                attack: 5,
                defence: 4,
                damage: (1, 3),
                health: 6,
                speed: 4
            },
            Self::SkeletonWarrior => CreatureStats {
                attack: 6,
                defence: 6,
                damage: (1, 3),
                health: 6,
                speed: 5
            },
            Self::WalkingDead => CreatureStats {
                attack: 5,
                defence: 5,
                damage: (2, 3),
                health: 15,
                speed: 3
            },
            Self::Zombie => CreatureStats {
                attack: 5,
                defence: 5,
                damage: (2, 3),
                health: 20,
                speed: 4
            },
            Self::Wight => CreatureStats {
                attack: 7,
                defence: 7,
                damage: (3, 5),
                health: 18,
                speed: 5
            },
            Self::Wraith => CreatureStats {
                attack: 7,
                defence: 7,
                damage: (3, 5),
                health: 18,
                speed: 7
            },
            Self::Vampire => CreatureStats {
                attack: 10,
                defence: 9,
                damage: (5, 8),
                health: 30,
                speed: 6
            },
            Self::VampireLord => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (5, 8),
                health: 40,
                speed: 9
            },
            Self::Lich => CreatureStats {
                attack: 13,
                defence: 10,
                damage: (11, 13),
                health: 30,
                speed: 6
            },
            Self::PowerLich => CreatureStats {
                attack: 13,
                defence: 10,
                damage: (11, 15),
                health: 40,
                speed: 7
            },
            Self::BlackKnight => CreatureStats {
                attack: 16,
                defence: 16,
                damage: (15, 30),
                health: 120,
                speed: 7
            },
            Self::DreadKnight => CreatureStats {
                attack: 18,
                defence: 18,
                damage: (15, 30),
                health: 120,
                speed: 9
            },
            Self::BoneDragon => CreatureStats {
                attack: 17,
                defence: 15,
                damage: (25, 50),
                health: 150,
                speed: 9
            },
            Self::GhostDragon => CreatureStats {
                attack: 19,
                defence: 17,
                damage: (25, 50),
                health: 200,
                speed: 14
            },
            // Dungeon
            Self::Troglodyte => CreatureStats {
                attack: 4,
                defence: 3,
                damage: (1, 3),
                health: 5,
                speed: 4
            },
            Self::InfernalTroglodyte => CreatureStats {
                attack: 5,
                defence: 4,
                damage: (1, 3),
                health: 6,
                speed: 5
            },
            Self::Harpy => CreatureStats {
                attack: 6,
                defence: 5,
                damage: (1, 4),
                health: 14,
                speed: 6
            },
            Self::HarpyHag => CreatureStats {
                attack: 6,
                defence: 6,
                damage: (1, 4),
                health: 14,
                speed: 9
            },
            Self::Beholder => CreatureStats {
                attack: 9,
                defence: 7,
                damage: (3, 5),
                health: 22,
                speed: 5
            },
            Self::EvilEye => CreatureStats {
                attack: 10,
                defence: 8,
                damage: (3, 5),
                health: 22,
                speed: 7
            },
            Self::Medusa => CreatureStats {
                attack: 9,
                defence: 9,
                damage: (6, 8),
                health: 25,
                speed: 5
            },
            Self::MedusaQueen => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (6, 8),
                health: 30,
                speed: 6
            },
            Self::Minotaur => CreatureStats {
                attack: 14,
                defence: 12,
                damage: (12, 20),
                health: 50,
                speed: 6
            },
            Self::MinotaurKing => CreatureStats {
                attack: 15,
                defence: 15,
                damage: (12, 20),
                health: 50,
                speed: 8
            },
            Self::Manticore => CreatureStats {
                attack: 15,
                defence: 13,
                damage: (14, 20),
                health: 80,
                speed: 7
            },
            Self::Scorpicore => CreatureStats {
                attack: 16,
                defence: 14,
                damage: (14, 20),
                health: 80,
                speed: 11
            },
            Self::RedDragon => CreatureStats {
                attack: 19,
                defence: 19,
                damage: (40, 50),
                health: 180,
                speed: 11
            },
            Self::BlackDragon => CreatureStats {
                attack: 25,
                defence: 25,
                damage: (40, 50),
                health: 300,
                speed: 15
            },
            // Stronghold
            Self::Goblin => CreatureStats {
                attack: 4,
                defence: 2,
                damage: (1, 2),
                health: 5,
                speed: 5
            },
            Self::Hobgoblin => CreatureStats {
                attack: 5,
                defence: 3,
                damage: (1, 2),
                health: 5,
                speed: 7
            },
            Self::WolfRider => CreatureStats {
                attack: 7,
                defence: 5,
                damage: (2, 4),
                health: 10,
                speed: 6
            },
            Self::WolfRaider => CreatureStats {
                attack: 8,
                defence: 5,
                damage: (3, 4),
                health: 10,
                speed: 8
            },
            Self::Orc => CreatureStats {
                attack: 8,
                defence: 4,
                damage: (2, 5),
                health: 15,
                speed: 4
            },
            Self::OrcChieftain => CreatureStats {
                attack: 8,
                defence: 4,
                damage: (2, 5),
                health: 20,
                speed: 5
            },
            Self::Ogre => CreatureStats {
                attack: 13,
                defence: 7,
                damage: (6, 12),
                health: 40,
                speed: 4
            },
            Self::OgreMagi => CreatureStats {
                attack: 13,
                defence: 7,
                damage: (6, 12),
                health: 60,
                speed: 5
            },
            Self::Roc => CreatureStats {
                attack: 13,
                defence: 11,
                damage: (11, 15),
                health: 60,
                speed: 7
            },
            Self::Thunderbird => CreatureStats {
                attack: 13,
                defence: 11,
                damage: (11, 15),
                health: 60,
                speed: 11
            },
            Self::Cyclops => CreatureStats {
                attack: 15,
                defence: 12,
                damage: (16, 20),
                health: 70,
                speed: 6
            },
            Self::CyclopsKing => CreatureStats {
                attack: 17,
                defence: 13,
                damage: (16, 20),
                health: 70,
                speed: 8
            },
            Self::Behemoth => CreatureStats {
                attack: 17,
                defence: 17,
                damage: (30, 50),
                health: 160,
                speed: 6
            },
            Self::AncientBehemoth => CreatureStats {
                attack: 19,
                defence: 19,
                damage: (30, 50),
                health: 300,
                speed: 9
            },
            // Fortress
            Self::Gnoll => CreatureStats {
                attack: 3,
                defence: 5,
                damage: (2, 3),
                health: 6,
                speed: 4
            },
            Self::GnollMarauder => CreatureStats {
                attack: 4,
                defence: 6,
                damage: (2, 3),
                health: 6,
                speed: 5
            },
            Self::Lizardman => CreatureStats {
                attack: 5,
                defence: 6,
                damage: (2, 3),
                health: 14,
                speed: 4
            },
            Self::LizardWarrior => CreatureStats {
                attack: 6,
                defence: 8,
                damage: (2, 5),
                health: 15,
                speed: 5
            },
            Self::SerpentFly => CreatureStats {
                attack: 7,
                defence: 9,
                damage: (2, 5),
                health: 20,
                speed: 9
            },
            Self::DragonFly => CreatureStats {
                attack: 8,
                defence: 10,
                damage: (2, 5),
                health: 20,
                speed: 13
            },
            Self::Basilisk => CreatureStats {
                attack: 11,
                defence: 11,
                damage: (6, 10),
                health: 35,
                speed: 5
            },
            Self::GreaterBasilisk => CreatureStats {
                attack: 12,
                defence: 12,
                damage: (6, 10),
                health: 40,
                speed: 7
            },
            Self::Gorgon => CreatureStats {
                attack: 10,
                defence: 14,
                damage: (12, 16),
                health: 70,
                speed: 5
            },
            Self::MightyGorgon => CreatureStats {
                attack: 11,
                defence: 16,
                damage: (12, 16),
                health: 70,
                speed: 6
            },
            Self::Wyvern => CreatureStats {
                attack: 14,
                defence: 14,
                damage: (14, 18),
                health: 70,
                speed: 7
            },
            Self::WyvernMonarch => CreatureStats {
                attack: 14,
                defence: 14,
                damage: (18, 22),
                health: 70,
                speed: 11
            },
            Self::Hydra => CreatureStats {
                attack: 16,
                defence: 18,
                damage: (25, 45),
                health: 175,
                speed: 5
            },
            Self::ChaosHydra => CreatureStats {
                attack: 18,
                defence: 20,
                damage: (25, 45),
                health: 250,
                speed: 7
            },
            // Conflux
            Self::Pixie => CreatureStats {
                attack: 2,
                defence: 2,
                damage: (1, 2),
                health: 3,
                speed: 7
            },
            Self::Sprite => CreatureStats {
                attack: 2,
                defence: 2,
                damage: (1, 3),
                health: 3,
                speed: 9
            },
            Self::AirElemental => CreatureStats {
                attack: 9,
                defence: 9,
                damage: (2, 8),
                health: 25,
                speed: 7
            },
            Self::StormElemental => CreatureStats {
                attack: 9,
                defence: 9,
                damage: (2, 8),
                health: 25,
                speed: 8
            },
            Self::WaterElemental => CreatureStats {
                attack: 8,
                defence: 10,
                damage: (3, 7),
                health: 30,
                speed: 5
            },
            Self::IceElemental => CreatureStats {
                attack: 8,
                defence: 10,
                damage: (3, 7),
                health: 30,
                speed: 6
            },
            Self::FireElemental => CreatureStats {
                attack: 10,
                defence: 8,
                damage: (4, 6),
                health: 35,
                speed: 6
            },
            Self::EnergyElemental => CreatureStats {
                attack: 12,
                defence: 8,
                damage: (4, 6),
                health: 35,
                speed: 8
            },
            Self::EarthElemental => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (4, 8),
                health: 40,
                speed: 4
            },
            Self::MagmaElemental => CreatureStats {
                attack: 11,
                defence: 11,
                damage: (6, 10),
                health: 40,
                speed: 6
            },
            Self::PsychicElemental => CreatureStats {
                attack: 15,
                defence: 13,
                damage: (10, 20),
                health: 75,
                speed: 7
            },
            Self::MagicElemental => CreatureStats {
                attack: 15,
                defence: 13,
                damage: (15, 25),
                health: 80,
                speed: 9
            },
            Self::Firebird => CreatureStats {
                attack: 18,
                defence: 18,
                damage: (30, 40),
                health: 150,
                speed: 15
            },
            Self::Phoenix => CreatureStats {
                attack: 21,
                defence: 18,
                damage: (30, 40),
                health: 200,
                speed: 21
            },
            // Neutral
            Self::Peasant => CreatureStats {
                attack: 1,
                defence: 1,
                damage: (1, 1),
                health: 1,
                speed: 3
            },
            Self::Halfling => CreatureStats {
                attack: 4,
                defence: 2,
                damage: (1, 3),
                health: 4,
                speed: 5
            },
            Self::Boar => CreatureStats {
                attack: 6,
                defence: 5,
                damage: (2, 3),
                health: 15,
                speed: 6
            },
            Self::Rogue => CreatureStats {
                attack: 8,
                defence: 3,
                damage: (2, 4),
                health: 10,
                speed: 6
            },
            Self::Mummy => CreatureStats {
                attack: 7,
                defence: 7,
                damage: (3, 5),
                health: 30,
                speed: 5
            },
            Self::Nomad => CreatureStats {
                attack: 9,
                defence: 8,
                damage: (2, 6),
                health: 30,
                speed: 7
            },
            Self::Sharpshooter => CreatureStats {
                attack: 12,
                defence: 10,
                damage: (8, 10),
                health: 15,
                speed: 9
            },
            Self::Troll => CreatureStats {
                attack: 14,
                defence: 7,
                damage: (10, 15),
                health: 40,
                speed: 7
            },
            Self::GoldGolem => CreatureStats {
                attack: 11,
                defence: 12,
                damage: (8, 10),
                health: 50,
                speed: 5
            },
            Self::DiamondGolem => CreatureStats {
                attack: 13,
                defence: 12,
                damage: (10, 14),
                health: 60,
                speed: 5
            },
            Self::Enchanter => CreatureStats {
                attack: 17,
                defence: 12,
                damage: (14, 14),
                health: 30,
                speed: 9
            },
            Self::FaerieDragon => CreatureStats {
                attack: 20,
                defence: 20,
                damage: (20, 30),
                health: 500,
                speed: 15
            },
            Self::RustDragon => CreatureStats {
                attack: 30,
                defence: 30,
                damage: (50, 50),
                health: 750,
                speed: 17
            },
            Self::CrystalDragon => CreatureStats {
                attack: 40,
                defence: 40,
                damage: (60, 75),
                health: 800,
                speed: 16
            },
            Self::AzureDragon => CreatureStats {
                attack: 50,
                defence: 50,
                damage: (70, 80),
                health: 1000,
                speed: 19
            },
            // War Machines
            Self::Ballista => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (2, 3),
                health: 250,
                speed: 0
            },
            Self::FirstAidTent => CreatureStats {
                attack: 0,
                defence: 0,
                damage: (0, 0),
                health: 75,
                speed: 0
            },
            Self::Catapult => CreatureStats {
                attack: 10,
                defence: 10,
                damage: (0, 0),
                health: 1000,
                speed: 0
            },
            Self::AmmoCart => CreatureStats {
                attack: 0,
                defence: 5,
                damage: (0, 0),
                health: 100,
                speed: 0
            }
        }
    }
}