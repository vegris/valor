use macros::EnumIndex;
use serde::Deserialize;
use strum_macros::{EnumCount, EnumIter};

use crate::{
    towns::Town,
    traits::{AnimationGroupT, ContainerType},
};

#[derive(Clone, Copy, Deserialize)]
pub enum Hero {
    Christian,
    Edric,
    Orrin,
    Sorsha,
    Sylvia,
    Tyris,
    Valeska,
    Catherine,
    HaartFresh,
    Roland,
    SirMullich,
    Adela,
    Adelaide,
    Caitlin,
    Cuthbert,
    Ingham,
    Loynis,
    Rion,
    Sanya,
    Clancy,
    Ivor,
    Jenova,
    Kyrre,
    Mephala,
    Ryland,
    Thorgrim,
    Ufretin,
    Gelu,
    Aeris,
    Alagar,
    Coronius,
    Elleshar,
    Gem,
    Malcom,
    Melodia,
    Uland,
    Fafner,
    Iona,
    Josephine,
    Neela,
    Piquedram,
    Rissa,
    Thane,
    Torosar,
    Aine,
    Astral,
    Cyra,
    Daremyth,
    Halon,
    Serena,
    Solmyr,
    Theodorus,
    Dracon,
    Calh,
    Fiona,
    Ignatius,
    Marius,
    Nymus,
    Octavia,
    Pyre,
    Rashka,
    Xeron,
    Ash,
    Axsis,
    Ayden,
    Calid,
    Olema,
    Xarfax,
    Xyron,
    Zydar,
    Charna,
    Clavius,
    Galthran,
    Isra,
    Moandor,
    Straker,
    Tamika,
    Vokial,
    HaartStale,
    Aislinn,
    Nagash,
    Nimbus,
    Sandro,
    Septienna,
    Thant,
    Vidomina,
    Xsi,
    Ajit,
    Arlach,
    Dace,
    Damacon,
    Gunnar,
    Lorelei,
    Shakti,
    Synca,
    Mutare,
    MutareDrake,
    Alamar,
    Darkstorn,
    Deemer,
    Geon,
    Jaegar,
    Jeddite,
    Malekith,
    Sephinroth,
    CragHack,
    Gretchin,
    Gurnisson,
    Jabarkas,
    Krellion,
    Shiva,
    Tyraxor,
    Yog,
    Boragus,
    Kilgor,
    Dessa,
    Gird,
    Gundula,
    Oris,
    Saurug,
    Terek,
    Vey,
    Zubin,
    Alkin,
    Broghild,
    Bron,
    Drakon,
    Gerwulf,
    Korbac,
    Tazar,
    Wystan,
    Andra,
    Merist,
    Mirlanda,
    Rosic,
    Styg,
    Tiva,
    Verdish,
    Voy,
    Adrienne,
    Erdamon,
    Fiur,
    Ignissa,
    Kalt,
    Lacus,
    Monere,
    Pasis,
    Thunar,
    Aenain,
    Brissa,
    Ciele,
    Gelare,
    Grindan,
    Inteus,
    Labetha,
    Luna,
}

pub enum Class {
    Knight,
    Cleric,
    Ranger,
    Druid,
    Alchemist,
    Wizard,
    Demoniac,
    Heretic,
    DeathKnight,
    Necromancer,
    Overlord,
    Warlock,
    Barbarian,
    BattleMage,
    Beastmaster,
    Witch,
    Planeswalker,
    Elementalist,
}

pub struct Stats {
    pub attack: i32,
    pub defence: i32,
    pub power: i32,
    pub knowledge: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Ability {
    AirMagic,
    Archery,
    Armorer,
    Artillery,
    Ballistics,
    Diplomacy,
    EagleEye,
    EarthMagic,
    Estates,
    FireMagic,
    FirstAid,
    Intelligence,
    Leadership,
    Learning,
    Logistics,
    Luck,
    Mysticism,
    Navigation,
    Necromancy,
    Offense,
    Pathfinding,
    Resistance,
    Scholar,
    Scouting,
    Sorcery,
    Tactics,
    WaterMagic,
    Wisdom,
}

#[derive(Clone, Copy)]
pub enum AbilityLevel {
    Basic,
    Advanced,
    Expert,
}

#[derive(Clone, Copy)]
pub struct LearnedAbility {
    pub ability: Ability,
    pub level: AbilityLevel,
}

#[derive(Clone, Copy, EnumCount, EnumIter, EnumIndex)]
pub enum Animation {
    Idle,
    Facepalm,
    Happy,
    Casting,
}

impl Hero {
    pub const fn class(self) -> Class {
        match self {
            Hero::Christian => Class::Knight,
            Hero::Edric => Class::Knight,
            Hero::Orrin => Class::Knight,
            Hero::Sorsha => Class::Knight,
            Hero::Sylvia => Class::Knight,
            Hero::Tyris => Class::Knight,
            Hero::Valeska => Class::Knight,
            Hero::Catherine => Class::Knight,
            Hero::HaartFresh => Class::Knight,
            Hero::Roland => Class::Knight,
            Hero::SirMullich => Class::Knight,
            Hero::Adela => Class::Cleric,
            Hero::Adelaide => Class::Cleric,
            Hero::Caitlin => Class::Cleric,
            Hero::Cuthbert => Class::Cleric,
            Hero::Ingham => Class::Cleric,
            Hero::Loynis => Class::Cleric,
            Hero::Rion => Class::Cleric,
            Hero::Sanya => Class::Cleric,

            Hero::Clancy => Class::Ranger,
            Hero::Ivor => Class::Ranger,
            Hero::Jenova => Class::Ranger,
            Hero::Kyrre => Class::Ranger,
            Hero::Mephala => Class::Ranger,
            Hero::Ryland => Class::Ranger,
            Hero::Thorgrim => Class::Ranger,
            Hero::Ufretin => Class::Ranger,
            Hero::Gelu => Class::Ranger,
            Hero::Aeris => Class::Druid,
            Hero::Alagar => Class::Druid,
            Hero::Coronius => Class::Druid,
            Hero::Elleshar => Class::Druid,
            Hero::Gem => Class::Druid,
            Hero::Malcom => Class::Druid,
            Hero::Melodia => Class::Druid,
            Hero::Uland => Class::Druid,

            Hero::Fafner => Class::Alchemist,
            Hero::Iona => Class::Alchemist,
            Hero::Josephine => Class::Alchemist,
            Hero::Neela => Class::Alchemist,
            Hero::Piquedram => Class::Alchemist,
            Hero::Rissa => Class::Alchemist,
            Hero::Thane => Class::Alchemist,
            Hero::Torosar => Class::Alchemist,
            Hero::Aine => Class::Wizard,
            Hero::Astral => Class::Wizard,
            Hero::Cyra => Class::Wizard,
            Hero::Daremyth => Class::Wizard,
            Hero::Halon => Class::Wizard,
            Hero::Serena => Class::Wizard,
            Hero::Solmyr => Class::Wizard,
            Hero::Theodorus => Class::Wizard,
            Hero::Dracon => Class::Wizard,

            Hero::Calh => Class::Demoniac,
            Hero::Fiona => Class::Demoniac,
            Hero::Ignatius => Class::Demoniac,
            Hero::Marius => Class::Demoniac,
            Hero::Nymus => Class::Demoniac,
            Hero::Octavia => Class::Demoniac,
            Hero::Pyre => Class::Demoniac,
            Hero::Rashka => Class::Demoniac,
            Hero::Xeron => Class::Demoniac,
            Hero::Ash => Class::Heretic,
            Hero::Axsis => Class::Heretic,
            Hero::Ayden => Class::Heretic,
            Hero::Calid => Class::Heretic,
            Hero::Olema => Class::Heretic,
            Hero::Xarfax => Class::Heretic,
            Hero::Xyron => Class::Heretic,
            Hero::Zydar => Class::Heretic,

            Hero::Charna => Class::DeathKnight,
            Hero::Clavius => Class::DeathKnight,
            Hero::Galthran => Class::DeathKnight,
            Hero::Isra => Class::DeathKnight,
            Hero::Moandor => Class::DeathKnight,
            Hero::Straker => Class::DeathKnight,
            Hero::Tamika => Class::DeathKnight,
            Hero::Vokial => Class::DeathKnight,
            Hero::HaartStale => Class::DeathKnight,
            Hero::Aislinn => Class::Necromancer,
            Hero::Nagash => Class::Necromancer,
            Hero::Nimbus => Class::Necromancer,
            Hero::Sandro => Class::Necromancer,
            Hero::Septienna => Class::Necromancer,
            Hero::Thant => Class::Necromancer,
            Hero::Vidomina => Class::Necromancer,
            Hero::Xsi => Class::Necromancer,

            Hero::Ajit => Class::Overlord,
            Hero::Arlach => Class::Overlord,
            Hero::Dace => Class::Overlord,
            Hero::Damacon => Class::Overlord,
            Hero::Gunnar => Class::Overlord,
            Hero::Lorelei => Class::Overlord,
            Hero::Shakti => Class::Overlord,
            Hero::Synca => Class::Overlord,
            Hero::Mutare => Class::Overlord,
            Hero::MutareDrake => Class::Overlord,
            Hero::Alamar => Class::Warlock,
            Hero::Darkstorn => Class::Warlock,
            Hero::Deemer => Class::Warlock,
            Hero::Geon => Class::Warlock,
            Hero::Jaegar => Class::Warlock,
            Hero::Jeddite => Class::Warlock,
            Hero::Malekith => Class::Warlock,
            Hero::Sephinroth => Class::Warlock,

            Hero::CragHack => Class::Barbarian,
            Hero::Gretchin => Class::Barbarian,
            Hero::Gurnisson => Class::Barbarian,
            Hero::Jabarkas => Class::Barbarian,
            Hero::Krellion => Class::Barbarian,
            Hero::Shiva => Class::Barbarian,
            Hero::Tyraxor => Class::Barbarian,
            Hero::Yog => Class::Barbarian,
            Hero::Boragus => Class::Barbarian,
            Hero::Kilgor => Class::Barbarian,
            Hero::Dessa => Class::BattleMage,
            Hero::Gird => Class::BattleMage,
            Hero::Gundula => Class::BattleMage,
            Hero::Oris => Class::BattleMage,
            Hero::Saurug => Class::BattleMage,
            Hero::Terek => Class::BattleMage,
            Hero::Vey => Class::BattleMage,
            Hero::Zubin => Class::BattleMage,

            Hero::Alkin => Class::Beastmaster,
            Hero::Broghild => Class::Beastmaster,
            Hero::Bron => Class::Beastmaster,
            Hero::Drakon => Class::Beastmaster,
            Hero::Gerwulf => Class::Beastmaster,
            Hero::Korbac => Class::Beastmaster,
            Hero::Tazar => Class::Beastmaster,
            Hero::Wystan => Class::Beastmaster,
            Hero::Andra => Class::Witch,
            Hero::Merist => Class::Witch,
            Hero::Mirlanda => Class::Witch,
            Hero::Rosic => Class::Witch,
            Hero::Styg => Class::Witch,
            Hero::Tiva => Class::Witch,
            Hero::Verdish => Class::Witch,
            Hero::Voy => Class::Witch,
            Hero::Adrienne => Class::Witch,

            Hero::Erdamon => Class::Planeswalker,
            Hero::Fiur => Class::Planeswalker,
            Hero::Ignissa => Class::Planeswalker,
            Hero::Kalt => Class::Planeswalker,
            Hero::Lacus => Class::Planeswalker,
            Hero::Monere => Class::Planeswalker,
            Hero::Pasis => Class::Planeswalker,
            Hero::Thunar => Class::Planeswalker,
            Hero::Aenain => Class::Elementalist,
            Hero::Brissa => Class::Elementalist,
            Hero::Ciele => Class::Elementalist,
            Hero::Gelare => Class::Elementalist,
            Hero::Grindan => Class::Elementalist,
            Hero::Inteus => Class::Elementalist,
            Hero::Labetha => Class::Elementalist,
            Hero::Luna => Class::Elementalist,
        }
    }

    pub const fn starting_abilities(self) -> [Option<LearnedAbility>; 2] {
        match self {
            Hero::Christian => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Edric => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Orrin => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Sorsha => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Sylvia => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Navigation,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Tyris => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Valeska => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Catherine => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::HaartFresh => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Estates,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Roland => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::SirMullich => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Adela => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Diplomacy,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Adelaide => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Caitlin => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Cuthbert => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Estates,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ingham => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Loynis => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Rion => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::FirstAid,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Sanya => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Clancy => [
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Pathfinding,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ivor => [
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Jenova => [
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Kyrre => [
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Logistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Mephala => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ryland => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Diplomacy,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Thorgrim => [
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Ufretin => [
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Luck,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gelu => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Aeris => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Alagar => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Coronius => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Elleshar => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gem => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::FirstAid,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Malcom => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Melodia => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Luck,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Uland => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Advanced,
                }),
                Some(LearnedAbility {
                    ability: Ability::Ballistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Fafner => [
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Iona => [
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Josephine => [
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Neela => [
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Piquedram => [
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Rissa => [
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Thane => [
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Torosar => [
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Aine => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Astral => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Cyra => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Diplomacy,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Daremyth => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Halon => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Serena => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Solmyr => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Theodorus => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Ballistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Dracon => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Calh => [
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Fiona => [
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Ignatius => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Marius => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Nymus => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Octavia => [
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Pyre => [
                Some(LearnedAbility {
                    ability: Ability::Logistics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Rashka => [
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Xeron => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ash => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Axsis => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ayden => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Calid => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Olema => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Ballistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Xarfax => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Xyron => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Zydar => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Charna => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Clavius => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Galthran => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Isra => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Moandor => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Straker => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Tamika => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Vokial => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::HaartStale => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Aislinn => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Nagash => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Nimbus => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Sandro => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Septienna => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Thant => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Vidomina => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Xsi => [
                Some(LearnedAbility {
                    ability: Ability::Necromancy,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ajit => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Arlach => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Dace => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Damacon => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Gunnar => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Logistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Lorelei => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Shakti => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Synca => [
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Mutare => [
                Some(LearnedAbility {
                    ability: Ability::Estates,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::MutareDrake => [
                Some(LearnedAbility {
                    ability: Ability::Estates,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Alamar => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scholar,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Darkstorn => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Deemer => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Advanced,
                }),
            ],
            Hero::Geon => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Jaegar => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Jeddite => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Malekith => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Sephinroth => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::CragHack => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Gretchin => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Pathfinding,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gurnisson => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Jabarkas => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Krellion => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Shiva => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Tyraxor => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Yog => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Ballistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Boragus => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Kilgor => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Dessa => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Logistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gird => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gundula => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Oris => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Saurug => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Terek => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Vey => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Zubin => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Alkin => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Broghild => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Scouting,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Bron => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Resistance,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Drakon => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Leadership,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gerwulf => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Korbac => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Pathfinding,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Tazar => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Wystan => [
                Some(LearnedAbility {
                    ability: Ability::Armorer,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Archery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Andra => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Intelligence,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Merist => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Mirlanda => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Rosic => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Mysticism,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Styg => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Sorcery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Tiva => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EagleEye,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Verdish => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::FirstAid,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Voy => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Navigation,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Adrienne => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::FireMagic,
                    level: AbilityLevel::Expert,
                }),
            ],
            Hero::Erdamon => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Estates,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Fiur => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Ignissa => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Kalt => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Learning,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Lacus => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Advanced,
                }),
                None,
            ],
            Hero::Monere => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Logistics,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Pasis => [
                Some(LearnedAbility {
                    ability: Ability::Offense,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Artillery,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Thunar => [
                Some(LearnedAbility {
                    ability: Ability::Tactics,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::Estates,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Aenain => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::AirMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Brissa => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::AirMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Ciele => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::WaterMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Gelare => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::WaterMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Grindan => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EarthMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Inteus => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::FireMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Labetha => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::EarthMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
            Hero::Luna => [
                Some(LearnedAbility {
                    ability: Ability::Wisdom,
                    level: AbilityLevel::Basic,
                }),
                Some(LearnedAbility {
                    ability: Ability::FireMagic,
                    level: AbilityLevel::Basic,
                }),
            ],
        }
    }
}

impl Class {
    pub const fn town(self) -> Town {
        match self {
            Class::Knight => Town::Castle,
            Class::Cleric => Town::Castle,
            Class::Ranger => Town::Rampart,
            Class::Druid => Town::Rampart,
            Class::Alchemist => Town::Tower,
            Class::Wizard => Town::Tower,
            Class::Demoniac => Town::Inferno,
            Class::Heretic => Town::Inferno,
            Class::DeathKnight => Town::Necropolis,
            Class::Necromancer => Town::Necropolis,
            Class::Overlord => Town::Dungeon,
            Class::Warlock => Town::Dungeon,
            Class::Barbarian => Town::Stronghold,
            Class::BattleMage => Town::Stronghold,
            Class::Beastmaster => Town::Fortress,
            Class::Witch => Town::Fortress,
            Class::Planeswalker => Town::Conflux,
            Class::Elementalist => Town::Conflux,
        }
    }

    pub const fn spritesheet_filename(self) -> &'static str {
        match self {
            Class::Knight => "CH00.def",
            Class::Cleric => "CH01.def",
            Class::Ranger => "CH02.def",
            Class::Druid => "CH03.def",
            Class::Alchemist => "CH04.def",
            Class::Wizard => "CH05.def",
            Class::Demoniac => "CH06.def",
            Class::Heretic => "CH07.def",
            Class::DeathKnight => "CH08.def",
            Class::Necromancer => "CH09.def",
            Class::Overlord => "CH010.def",
            Class::Warlock => "CH011.def",
            Class::Barbarian => "CH012.def",
            Class::BattleMage => "CH013.def",
            Class::Beastmaster => "CH014.def",
            Class::Witch => "CH015.def",
            Class::Planeswalker => "CH016.def",
            Class::Elementalist => "CH017.def",
        }
    }

    pub const fn starting_stats(self) -> Stats {
        match self {
            Class::Knight => Stats {
                attack: 2,
                defence: 2,
                power: 1,
                knowledge: 1,
            },
            Class::Cleric => Stats {
                attack: 1,
                defence: 0,
                power: 2,
                knowledge: 2,
            },
            Class::Ranger => Stats {
                attack: 1,
                defence: 3,
                power: 1,
                knowledge: 1,
            },
            Class::Druid => Stats {
                attack: 0,
                defence: 2,
                power: 1,
                knowledge: 2,
            },
            Class::Alchemist => Stats {
                attack: 1,
                defence: 1,
                power: 2,
                knowledge: 2,
            },
            Class::Wizard => Stats {
                attack: 0,
                defence: 0,
                power: 2,
                knowledge: 3,
            },
            Class::Demoniac => Stats {
                attack: 2,
                defence: 2,
                power: 1,
                knowledge: 1,
            },
            Class::Heretic => Stats {
                attack: 1,
                defence: 1,
                power: 2,
                knowledge: 1,
            },
            Class::DeathKnight => Stats {
                attack: 1,
                defence: 2,
                power: 2,
                knowledge: 1,
            },
            Class::Necromancer => Stats {
                attack: 1,
                defence: 0,
                power: 2,
                knowledge: 2,
            },
            Class::Overlord => Stats {
                attack: 2,
                defence: 2,
                power: 1,
                knowledge: 1,
            },
            Class::Warlock => Stats {
                attack: 0,
                defence: 0,
                power: 3,
                knowledge: 2,
            },
            Class::Barbarian => Stats {
                attack: 4,
                defence: 0,
                power: 1,
                knowledge: 1,
            },
            Class::BattleMage => Stats {
                attack: 2,
                defence: 1,
                power: 1,
                knowledge: 1,
            },
            Class::Beastmaster => Stats {
                attack: 0,
                defence: 4,
                power: 1,
                knowledge: 1,
            },
            Class::Witch => Stats {
                attack: 0,
                defence: 1,
                power: 2,
                knowledge: 2,
            },
            Class::Planeswalker => Stats {
                attack: 3,
                defence: 1,
                power: 1,
                knowledge: 1,
            },
            Class::Elementalist => Stats {
                attack: 0,
                defence: 0,
                power: 3,
                knowledge: 3,
            },
        }
    }
}

impl ContainerType for Animation {
    const CONTAINER_TYPE: u32 = 73;
}

impl AnimationGroupT for Animation {
    fn container_index(self) -> u32 {
        match self {
            Self::Idle => 1,
            Self::Facepalm => 2,
            Self::Happy => 3,
            Self::Casting => 4,
        }
    }
}
