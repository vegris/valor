use super::Hero;

pub enum Level {
    Basic,
    Advanced,
    Expert,
}

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
    Interference,
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

impl Hero {
    pub const fn starting_abilities(self) -> [Option<(Ability, Level)>; 2] {
        match self {
            Hero::Christian => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Edric => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Armorer, Level::Basic)),
            ],
            Hero::Orrin => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Archery, Level::Basic)),
            ],
            Hero::Sorsha => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Sylvia => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Navigation, Level::Basic)),
            ],
            Hero::Tyris => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Valeska => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Archery, Level::Basic)),
            ],
            Hero::Catherine => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::HaartFresh => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Estates, Level::Basic)),
            ],
            Hero::Roland => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Armorer, Level::Basic)),
            ],
            Hero::SirMullich => [Some((Ability::Leadership, Level::Advanced)), None],
            Hero::Adela => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Diplomacy, Level::Basic)),
            ],
            Hero::Adelaide => [Some((Ability::Wisdom, Level::Advanced)), None],
            Hero::Caitlin => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Cuthbert => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Estates, Level::Basic)),
            ],
            Hero::Ingham => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Mysticism, Level::Basic)),
            ],
            Hero::Loynis => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],
            Hero::Rion => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::FirstAid, Level::Basic)),
            ],
            Hero::Sanya => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],

            Hero::Clancy => [
                Some((Ability::Resistance, Level::Basic)),
                Some((Ability::Pathfinding, Level::Basic)),
            ],
            Hero::Ivor => [
                Some((Ability::Archery, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Jenova => [Some((Ability::Archery, Level::Advanced)), None],
            Hero::Kyrre => [
                Some((Ability::Archery, Level::Basic)),
                Some((Ability::Logistics, Level::Basic)),
            ],
            Hero::Mephala => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Armorer, Level::Basic)),
            ],
            Hero::Ryland => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Diplomacy, Level::Basic)),
            ],
            Hero::Thorgrim => [Some((Ability::Resistance, Level::Advanced)), None],
            Hero::Ufretin => [
                Some((Ability::Resistance, Level::Basic)),
                Some((Ability::Luck, Level::Basic)),
            ],
            Hero::Gelu => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Archery, Level::Basic)),
            ],
            Hero::Aeris => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Scouting, Level::Basic)),
            ],
            Hero::Alagar => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Coronius => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Scholar, Level::Basic)),
            ],
            Hero::Elleshar => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Gem => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::FirstAid, Level::Basic)),
            ],
            Hero::Malcom => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Melodia => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Luck, Level::Basic)),
            ],
            Hero::Uland => [
                Some((Ability::Wisdom, Level::Advanced)),
                Some((Ability::Ballistics, Level::Basic)),
            ],

            Hero::Fafner => [
                Some((Ability::Scholar, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Iona => [
                Some((Ability::Scholar, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Josephine => [
                Some((Ability::Mysticism, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Neela => [
                Some((Ability::Scholar, Level::Basic)),
                Some((Ability::Armorer, Level::Basic)),
            ],
            Hero::Piquedram => [
                Some((Ability::Mysticism, Level::Basic)),
                Some((Ability::Scouting, Level::Basic)),
            ],
            Hero::Rissa => [
                Some((Ability::Mysticism, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Thane => [Some((Ability::Scholar, Level::Advanced)), None],
            Hero::Torosar => [
                Some((Ability::Mysticism, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Aine => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Scholar, Level::Basic)),
            ],
            Hero::Astral => [Some((Ability::Wisdom, Level::Advanced)), None],
            Hero::Cyra => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Diplomacy, Level::Basic)),
            ],
            Hero::Daremyth => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Halon => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Mysticism, Level::Basic)),
            ],
            Hero::Serena => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Solmyr => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Theodorus => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Ballistics, Level::Basic)),
            ],
            Hero::Dracon => [Some((Ability::Wisdom, Level::Advanced)), None],

            Hero::Calh => [
                Some((Ability::Archery, Level::Basic)),
                Some((Ability::Scouting, Level::Basic)),
            ],
            Hero::Fiona => [Some((Ability::Scouting, Level::Advanced)), None],
            Hero::Ignatius => [
                Some((Ability::Tactics, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Marius => [Some((Ability::Armorer, Level::Advanced)), None],
            Hero::Nymus => [Some((Ability::Offense, Level::Advanced)), None],
            Hero::Octavia => [
                Some((Ability::Scholar, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Pyre => [
                Some((Ability::Logistics, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Rashka => [
                Some((Ability::Scholar, Level::Basic)),
                Some((Ability::Wisdom, Level::Basic)),
            ],
            Hero::Xeron => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Ash => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Axsis => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Mysticism, Level::Basic)),
            ],
            Hero::Ayden => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Calid => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],
            Hero::Olema => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Ballistics, Level::Basic)),
            ],
            Hero::Xarfax => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Leadership, Level::Basic)),
            ],
            Hero::Xyron => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Scholar, Level::Basic)),
            ],
            Hero::Zydar => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],

            Hero::Charna => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Clavius => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Galthran => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Armorer, Level::Basic)),
            ],
            Hero::Isra => [Some((Ability::Necromancy, Level::Advanced)), None],
            Hero::Moandor => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],
            Hero::Straker => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Tamika => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Vokial => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::HaartStale => [Some((Ability::Necromancy, Level::Advanced)), None],
            Hero::Aislinn => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Wisdom, Level::Basic)),
            ],
            Hero::Nagash => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Nimbus => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Sandro => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Septienna => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Scholar, Level::Basic)),
            ],
            Hero::Thant => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Mysticism, Level::Basic)),
            ],
            Hero::Vidomina => [Some((Ability::Necromancy, Level::Advanced)), None],
            Hero::Xsi => [
                Some((Ability::Necromancy, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],

            Hero::Ajit => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Arlach => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Dace => [
                Some((Ability::Tactics, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Damacon => [Some((Ability::Offense, Level::Advanced)), None],
            Hero::Gunnar => [
                Some((Ability::Tactics, Level::Basic)),
                Some((Ability::Logistics, Level::Basic)),
            ],
            Hero::Lorelei => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Scouting, Level::Basic)),
            ],
            Hero::Shakti => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Synca => [
                Some((Ability::Leadership, Level::Basic)),
                Some((Ability::Scholar, Level::Basic)),
            ],
            Hero::Mutare => [
                Some((Ability::Estates, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::MutareDrake => [
                Some((Ability::Estates, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Alamar => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Scholar, Level::Basic)),
            ],
            Hero::Darkstorn => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],
            Hero::Deemer => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Scouting, Level::Advanced)),
            ],
            Hero::Geon => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Jaegar => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Mysticism, Level::Basic)),
            ],
            Hero::Jeddite => [Some((Ability::Wisdom, Level::Advanced)), None],
            Hero::Malekith => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Sephinroth => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],

            Hero::CragHack => [Some((Ability::Offense, Level::Advanced)), None],
            Hero::Gretchin => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Pathfinding, Level::Basic)),
            ],
            Hero::Gurnisson => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Jabarkas => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Archery, Level::Basic)),
            ],
            Hero::Krellion => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Shiva => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Scouting, Level::Basic)),
            ],
            Hero::Tyraxor => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Yog => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Ballistics, Level::Basic)),
            ],
            Hero::Boragus => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Kilgor => [Some((Ability::Offense, Level::Advanced)), None],
            Hero::Dessa => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Logistics, Level::Basic)),
            ],
            Hero::Gird => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Gundula => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Oris => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Saurug => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Terek => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Tactics, Level::Basic)),
            ],
            Hero::Vey => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Leadership, Level::Basic)),
            ],
            Hero::Zubin => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],

            Hero::Alkin => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Offense, Level::Basic)),
            ],
            Hero::Broghild => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Scouting, Level::Basic)),
            ],
            Hero::Bron => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Resistance, Level::Basic)),
            ],
            Hero::Drakon => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Leadership, Level::Basic)),
            ],
            Hero::Gerwulf => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Korbac => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Pathfinding, Level::Basic)),
            ],
            Hero::Tazar => [Some((Ability::Armorer, Level::Advanced)), None],
            Hero::Wystan => [
                Some((Ability::Armorer, Level::Basic)),
                Some((Ability::Archery, Level::Basic)),
            ],
            Hero::Andra => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Intelligence, Level::Basic)),
            ],
            Hero::Merist => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],
            Hero::Mirlanda => [Some((Ability::Wisdom, Level::Advanced)), None],
            Hero::Rosic => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Mysticism, Level::Basic)),
            ],
            Hero::Styg => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Sorcery, Level::Basic)),
            ],
            Hero::Tiva => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EagleEye, Level::Basic)),
            ],
            Hero::Verdish => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::FirstAid, Level::Basic)),
            ],
            Hero::Voy => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::Navigation, Level::Basic)),
            ],
            Hero::Adrienne => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::FireMagic, Level::Expert)),
            ],

            Hero::Erdamon => [
                Some((Ability::Tactics, Level::Basic)),
                Some((Ability::Estates, Level::Basic)),
            ],
            Hero::Fiur => [Some((Ability::Offense, Level::Advanced)), None],
            Hero::Ignissa => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Kalt => [
                Some((Ability::Tactics, Level::Basic)),
                Some((Ability::Learning, Level::Basic)),
            ],
            Hero::Lacus => [Some((Ability::Tactics, Level::Advanced)), None],
            Hero::Monere => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Logistics, Level::Basic)),
            ],
            Hero::Pasis => [
                Some((Ability::Offense, Level::Basic)),
                Some((Ability::Artillery, Level::Basic)),
            ],
            Hero::Thunar => [
                Some((Ability::Tactics, Level::Basic)),
                Some((Ability::Estates, Level::Basic)),
            ],
            Hero::Aenain => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::AirMagic, Level::Basic)),
            ],
            Hero::Brissa => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::AirMagic, Level::Basic)),
            ],
            Hero::Ciele => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::WaterMagic, Level::Basic)),
            ],
            Hero::Gelare => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::WaterMagic, Level::Basic)),
            ],
            Hero::Grindan => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EarthMagic, Level::Basic)),
            ],
            Hero::Inteus => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::FireMagic, Level::Basic)),
            ],
            Hero::Labetha => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::EarthMagic, Level::Basic)),
            ],
            Hero::Luna => [
                Some((Ability::Wisdom, Level::Basic)),
                Some((Ability::FireMagic, Level::Basic)),
            ],
        }
    }
}
