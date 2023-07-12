use super::Stats;
use crate::towns::Town;

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
