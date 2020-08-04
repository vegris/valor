#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Creature {
    Champion,
    Peasant,
    Beholder
}

impl Creature {
    pub fn filename(self) -> &'static str {
        FILENAMES[self as usize]
    }
    pub const fn count() -> usize {
        3
    }
}

const FILENAMES: [&str; Creature::count()] = [
    "CCHAMP.def",
    "Cpeas.def",
    "cbehol.def"
];
