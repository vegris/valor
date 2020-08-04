#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Creature {
    Champion,
    Peasant
}

impl Creature {
    pub fn filename(self) -> &'static str {
        FILENAMES[self as usize]
    }
    pub const fn count() -> usize {
        2
    }
}

const FILENAMES: [&str; Creature::count()] = [
    "CCHAMP.def",
    "Cpeas.def"
];
