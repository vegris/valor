use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum Cursor {
    Forbidden,
    Run,
    Fly,
    Arrow,
    Hero,
    Question,
    Pointer,
    AttackDownLeft,
    AttackLeft,
    AttackUpLeft,
    AttackUpRight,
    AttackRight,
    AttackDownRight,
    AttackDown,
    AttackUp,
    ArrowBroken,
    Catapult,
    Heal,
    Sacrifice,
    Teleport,
}

impl Cursor {
    pub const CONTAINTER_FILENAME: &str = "CRCOMBAT.def";
    pub const CONTAINER_TYPE: u32 = 70;

    pub const fn pointer_offset(self) -> (i32, i32) {
        match self {
            Self::Forbidden => (12, 12),
            Self::Run => (8, 8),
            Self::Fly => (12, 10),
            Self::Arrow => (12, 10),
            Self::Hero => (10, 10),
            Self::Question => (8, 10),
            Self::Pointer => (1, 2),
            Self::Catapult => (12, 10),
            Self::Heal => (12, 10),
            Self::Sacrifice => (12, 10),
            Self::Teleport => (12, 12),

            Self::AttackDownLeft => (21, 0),
            Self::AttackLeft => (31, 6),
            Self::AttackUpLeft => (21, 21),
            Self::AttackUpRight => (0, 21),
            Self::AttackRight => (0, 6),
            Self::AttackDownRight => (0, 0),
            Self::AttackDown => (6, 0),
            Self::AttackUp => (6, 16),

            _ => (0, 0),
        }
    }
}
