pub enum Misc {
    CellGrid,
    CellGridShadow
}

impl Misc {
    pub const fn filename(self) -> &'static str {
        match self {
            Misc::CellGrid => "CCellGrd.pcx",
            Misc::CellGridShadow => "CCellShd.pcx"
        }
    }
}
