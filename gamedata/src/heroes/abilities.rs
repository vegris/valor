use super::Hero;

#[derive(Clone, Copy)]
pub enum Level {
    Basic,
    Advanced,
    Expert,
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
pub struct HeroAbility {
    pub ability: Ability,
    pub level: Level,
}

impl Hero {
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
