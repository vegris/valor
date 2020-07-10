pub enum Misc {
    CellGrid,
    CellGridShadow
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
