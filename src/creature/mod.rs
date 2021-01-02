use num_traits::{ToPrimitive, FromPrimitive};

mod abilities;
mod stats;
mod factions;
mod spritesheets;

pub use abilities::CreatureAbility;
pub use stats::CreatureStats;
pub use factions::Faction;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, ToPrimitive, FromPrimitive)]
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
    // Tower
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
    // Stronghold
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
    // Fortress
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
    AmmoCart,
}

type C = Creature;

impl Creature {
    pub const COUNT: usize = 145;

    pub fn is_wide(&self) -> bool {
        const WIDE_CREATURES: [Creature; 53] = [
            // Castle
            C::Griffin,
            C::RoyalGriffin,
            C::Cavalier,
            C::Champion,
            C::Archangel,
            // Rampart
            C::Centaur,
            C::CentaurCaptain,
            C::Pegasus,
            C::SilverPegasus,
            C::Unicorn,
            C::WarUnicorn,
            C::GreenDragon,
            C::GoldDragon,
            // Tower
            C::Naga,
            C::NagaQueen,
            // Inferno
            C::HellHound,
            C::Cerberus,
            // Necropolis
            C::BlackKnight,
            C::DreadKnight,
            C::BoneDragon,
            C::GhostDragon,
            // Dungeon
            C::Medusa,
            C::MedusaQueen,
            C::Manticore,
            C::Scorpicore,
            C::RedDragon,
            C::BlackDragon,
            // Stronghold
            C::WolfRider,
            C::WolfRaider,
            C::Roc,
            C::Thunderbird,
            C::Behemoth,
            C::AncientBehemoth,
            // Fortress
            C::Basilisk,
            C::GreaterBasilisk,
            C::Gorgon,
            C::MightyGorgon,
            C::Wyvern,
            C::WyvernMonarch,
            C::Hydra,
            C::ChaosHydra,
            // Conflux
            C::WaterElemental,
            C::IceElemental,
            C::Firebird,
            C::Phoenix,
            // Neutral
            C::Boar,
            C::Nomad,
            C::FaerieDragon,
            C::RustDragon,
            C::CrystalDragon,
            C::AzureDragon,
            // War Machines
            C::Ballista,
            C::Catapult,
        ];
        WIDE_CREATURES.contains(self)
    }

    pub fn is_flying(&self) -> bool {
        const FLYING_CREATURES: [Creature; 42] = [
            // Castle
            C::Griffin,
            C::RoyalGriffin,
            C::Angel,
            C::Archangel,
            // Rampart
            C::Pegasus,
            C::SilverPegasus,
            C::GreenDragon,
            C::GoldDragon,
            // Tower
            C::StoneGargoyle,
            C::ObsidianGargoyle,
            C::Genie,
            C::MasterGenie,
            // Inferno
            C::Efreeti,
            C::EfreetSultan,
            C::Devil,
            C::ArchDevil,
            // Necropolis
            C::Wight,
            C::Wraith,
            C::Vampire,
            C::VampireLord,
            C::BoneDragon,
            C::GhostDragon,
            // Dungeon
            C::Harpy,
            C::HarpyHag,
            C::Manticore,
            C::Scorpicore,
            C::RedDragon,
            C::BlackDragon,
            // Stronghold
            C::Roc,
            C::Thunderbird,
            // Fortress
            C::SerpentFly,
            C::DragonFly,
            C::Wyvern,
            C::WyvernMonarch,
            // Conflux
            C::Pixie,
            C::Sprite,
            C::EnergyElemental,
            C::Firebird,
            C::Phoenix,
            // Neutral
            C::FaerieDragon,
            C::RustDragon,
            C::AzureDragon,
        ];
        FLYING_CREATURES.contains(self)
    }

    pub fn is_undead(&self) -> bool {
        const NON_NECROPOLIS_UNDEAD: [Creature; 1] = [C::Mummy];
        self.faction() == Faction::Necropolis || NON_NECROPOLIS_UNDEAD.contains(self)
    }

    pub fn upgrades_to(&self) -> Option<Creature> {
        if [Faction::Neutral, Faction::WarMachines].contains(&self.faction()) {
            return None
        }
        
        let creature_id = self.to_u32().unwrap();

        if creature_id % 2 == 0 {
            let upgrade_id = Creature::from_u32(creature_id + 1).unwrap();
            Some(upgrade_id)
        } else {
            None
        }
    }

    pub fn upgrade_of(&self) -> Option<Creature> {
        if [Faction::Neutral, Faction::WarMachines].contains(&self.faction()) {
            return None
        }
        
        let creature_id = self.to_u32().unwrap();

        if creature_id % 2 == 1 {
            let parent_id = Creature::from_u32(creature_id - 1).unwrap();
            Some(parent_id)
        } else {
            None
        }
    }
}