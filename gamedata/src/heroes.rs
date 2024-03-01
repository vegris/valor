use serde::Deserialize;

use crate::towns::Town;

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
pub enum Level {
    Basic,
    Advanced,
    Expert,
}

#[derive(Clone, Copy)]
pub struct HeroAbility {
    pub ability: Ability,
    pub level: Level,
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

    pub const fn starting_abilities(self) -> [Option<HeroAbility>; 2] {
        match self {
            Hero::Christian => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Edric => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
            ],
            Hero::Orrin => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
            ],
            Hero::Sorsha => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Sylvia => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Navigation,
                    level: Level::Basic,
                }),
            ],
            Hero::Tyris => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Valeska => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
            ],
            Hero::Catherine => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::HaartFresh => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Estates,
                    level: Level::Basic,
                }),
            ],
            Hero::Roland => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
            ],
            Hero::SirMullich => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Adela => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Diplomacy,
                    level: Level::Basic,
                }),
            ],
            Hero::Adelaide => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Caitlin => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Cuthbert => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Estates,
                    level: Level::Basic,
                }),
            ],
            Hero::Ingham => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
            ],
            Hero::Loynis => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Rion => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::FirstAid,
                    level: Level::Basic,
                }),
            ],
            Hero::Sanya => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Clancy => [
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Pathfinding,
                    level: Level::Basic,
                }),
            ],
            Hero::Ivor => [
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Jenova => [
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Kyrre => [
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Logistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Mephala => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
            ],
            Hero::Ryland => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Diplomacy,
                    level: Level::Basic,
                }),
            ],
            Hero::Thorgrim => [
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Ufretin => [
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Luck,
                    level: Level::Basic,
                }),
            ],
            Hero::Gelu => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
            ],
            Hero::Aeris => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Basic,
                }),
            ],
            Hero::Alagar => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Coronius => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
            ],
            Hero::Elleshar => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Gem => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::FirstAid,
                    level: Level::Basic,
                }),
            ],
            Hero::Malcom => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Melodia => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Luck,
                    level: Level::Basic,
                }),
            ],
            Hero::Uland => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Advanced,
                }),
                Some(HeroAbility {
                    ability: Ability::Ballistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Fafner => [
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Iona => [
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Josephine => [
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Neela => [
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
            ],
            Hero::Piquedram => [
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Basic,
                }),
            ],
            Hero::Rissa => [
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Thane => [
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Torosar => [
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Aine => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
            ],
            Hero::Astral => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Cyra => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Diplomacy,
                    level: Level::Basic,
                }),
            ],
            Hero::Daremyth => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Halon => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
            ],
            Hero::Serena => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Solmyr => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Theodorus => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Ballistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Dracon => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Calh => [
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Basic,
                }),
            ],
            Hero::Fiona => [
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Ignatius => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Marius => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Nymus => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Octavia => [
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Pyre => [
                Some(HeroAbility {
                    ability: Ability::Logistics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Rashka => [
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
            ],
            Hero::Xeron => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Ash => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Axsis => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
            ],
            Hero::Ayden => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Calid => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Olema => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Ballistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Xarfax => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
            ],
            Hero::Xyron => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
            ],
            Hero::Zydar => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Charna => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Clavius => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Galthran => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
            ],
            Hero::Isra => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Moandor => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Straker => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Tamika => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Vokial => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::HaartStale => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Aislinn => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
            ],
            Hero::Nagash => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Nimbus => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Sandro => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Septienna => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
            ],
            Hero::Thant => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
            ],
            Hero::Vidomina => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Xsi => [
                Some(HeroAbility {
                    ability: Ability::Necromancy,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Ajit => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Arlach => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Dace => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Damacon => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Gunnar => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Logistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Lorelei => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Basic,
                }),
            ],
            Hero::Shakti => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Synca => [
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
            ],
            Hero::Mutare => [
                Some(HeroAbility {
                    ability: Ability::Estates,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::MutareDrake => [
                Some(HeroAbility {
                    ability: Ability::Estates,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Alamar => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scholar,
                    level: Level::Basic,
                }),
            ],
            Hero::Darkstorn => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Deemer => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Advanced,
                }),
            ],
            Hero::Geon => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Jaegar => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
            ],
            Hero::Jeddite => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Malekith => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Sephinroth => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::CragHack => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Gretchin => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Pathfinding,
                    level: Level::Basic,
                }),
            ],
            Hero::Gurnisson => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Jabarkas => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
            ],
            Hero::Krellion => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Shiva => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Basic,
                }),
            ],
            Hero::Tyraxor => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Yog => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Ballistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Boragus => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Kilgor => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Dessa => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Logistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Gird => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Gundula => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Oris => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Saurug => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Terek => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
            ],
            Hero::Vey => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
            ],
            Hero::Zubin => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Alkin => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
            ],
            Hero::Broghild => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Scouting,
                    level: Level::Basic,
                }),
            ],
            Hero::Bron => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Resistance,
                    level: Level::Basic,
                }),
            ],
            Hero::Drakon => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Leadership,
                    level: Level::Basic,
                }),
            ],
            Hero::Gerwulf => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Korbac => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Pathfinding,
                    level: Level::Basic,
                }),
            ],
            Hero::Tazar => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Wystan => [
                Some(HeroAbility {
                    ability: Ability::Armorer,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Archery,
                    level: Level::Basic,
                }),
            ],
            Hero::Andra => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Intelligence,
                    level: Level::Basic,
                }),
            ],
            Hero::Merist => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Mirlanda => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Rosic => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Mysticism,
                    level: Level::Basic,
                }),
            ],
            Hero::Styg => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Sorcery,
                    level: Level::Basic,
                }),
            ],
            Hero::Tiva => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EagleEye,
                    level: Level::Basic,
                }),
            ],
            Hero::Verdish => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::FirstAid,
                    level: Level::Basic,
                }),
            ],
            Hero::Voy => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Navigation,
                    level: Level::Basic,
                }),
            ],
            Hero::Adrienne => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::FireMagic,
                    level: Level::Expert,
                }),
            ],
            Hero::Erdamon => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Estates,
                    level: Level::Basic,
                }),
            ],
            Hero::Fiur => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Ignissa => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Kalt => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Learning,
                    level: Level::Basic,
                }),
            ],
            Hero::Lacus => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Advanced,
                }),
                None,
            ],
            Hero::Monere => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Logistics,
                    level: Level::Basic,
                }),
            ],
            Hero::Pasis => [
                Some(HeroAbility {
                    ability: Ability::Offense,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Artillery,
                    level: Level::Basic,
                }),
            ],
            Hero::Thunar => [
                Some(HeroAbility {
                    ability: Ability::Tactics,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::Estates,
                    level: Level::Basic,
                }),
            ],
            Hero::Aenain => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::AirMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Brissa => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::AirMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Ciele => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::WaterMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Gelare => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::WaterMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Grindan => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EarthMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Inteus => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::FireMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Labetha => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::EarthMagic,
                    level: Level::Basic,
                }),
            ],
            Hero::Luna => [
                Some(HeroAbility {
                    ability: Ability::Wisdom,
                    level: Level::Basic,
                }),
                Some(HeroAbility {
                    ability: Ability::FireMagic,
                    level: Level::Basic,
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
