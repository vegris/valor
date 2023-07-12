use gamedata::creatures::Creature;

use super::hexagon_part::HexagonPart;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackDirection {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
}

impl AttackDirection {
    pub fn from_hexagon_part(hexagon_part: HexagonPart, attacking_creature: Creature) -> Self {
        match (hexagon_part, attacking_creature.is_wide()) {
            (HexagonPart::Left, _) => Self::Left,
            (HexagonPart::Right, _) => Self::Right,
            (HexagonPart::TopHalfLeft, false) => Self::TopLeft,
            (HexagonPart::TopHalfLeft, true) => Self::Top,
            (HexagonPart::TopHalfRight, false) => Self::TopRight,
            (HexagonPart::TopHalfRight, true) => Self::Top,
            (HexagonPart::BotHalfLeft, false) => Self::BottomLeft,
            (HexagonPart::BotHalfLeft, true) => Self::Bottom,
            (HexagonPart::BotHalfRight, false) => Self::BottomRight,
            (HexagonPart::BotHalfRight, true) => Self::Bottom,
            (HexagonPart::BotLeft, _) => Self::BottomLeft,
            (HexagonPart::BotRight, _) => Self::BottomRight,
            (HexagonPart::TopLeft, _) => Self::TopLeft,
            (HexagonPart::TopRight, _) => Self::TopRight,
        }
    }
}
