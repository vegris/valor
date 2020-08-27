#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Creature {
    Champion,
    Peasant,
    Beholder,
    Enchanter,
    DeathKnight,
    Ent
}

impl Creature {
    pub fn filename(self) -> &'static str {
        FILENAMES[self as usize]
    }
    pub const fn count() -> usize {
        6
    }
}

const FILENAMES: [&str; Creature::count()] = [
    "CCHAMP.def",
    "Cpeas.def",
    "cbehol.def",
    "Cench.def",
    "CBKNIG.def",
    "CBTREE.def"
];
