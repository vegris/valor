#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Creature {
    Champion,
    Peasant,
    Beholder,
    Enchanter,
    BlackKnight,
    Ent
}

impl Creature {
    pub const COUNT: usize = 6;

    pub const fn filename(self) -> &'static str {
        match self {
            Creature::Champion => "CCHAMP.def",
            Creature::Peasant => "Cpeas.def",
            Creature::Beholder => "cbehol.def",
            Creature::Enchanter => "Cench.def",
            Creature::BlackKnight => "CBKNIG.def",
            Creature::Ent => "CBTREE.def"
        }
    }
}
