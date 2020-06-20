#[derive(Clone, Copy)]
pub enum Creature {
    Champion,
    Peasant
}

impl Creature {
    pub fn filename(self) -> &'static str {
        FILENAMES[self as usize]
    }
}

const FILENAMES: [&str; 2] = [
    "CCHAMP.def",
    "Cpeas.def"
];