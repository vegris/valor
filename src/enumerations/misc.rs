pub enum Misc {
    CellGrid,
    CellGridShadowed
}

impl Misc {
    pub fn filename(self) -> &'static str {
        FILENAMES[self as usize]
    }
    pub const fn count() -> usize {
        2
    }
}

const FILENAMES: [&str; Misc::count()] = [
    "CCellGrd.pcx",
    "CCellShd.pcx"
];
