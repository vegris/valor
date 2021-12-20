use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum HexagonPart {
    Left,
    TopLeft,
    TopHalfLeft,
    TopHalfRight,
    TopRight,
    Right,
    BotRight,
    BotHalfRight,
    BotHalfLeft,
    BotLeft
}

impl HexagonPart {
    // Конец дуги соответствующей части
    // если идти по часовой стрелке
    fn arc_end(&self) -> f32 {
        use std::f32::consts::*;
        // [0; 2*PI]
        // Ноль - середина левой стороны
        // Идём по часовой стрелке
        match self {
            Self::Left         => -(PI - FRAC_2_PI),
            Self::TopLeft      => -(FRAC_PI_2 + FRAC_2_PI),
            Self::TopHalfLeft  => -FRAC_PI_2,
            Self::TopHalfRight => -(FRAC_PI_2 - FRAC_2_PI),
            Self::TopRight     => -FRAC_2_PI,
            Self::Right        =>  FRAC_2_PI,
            Self::BotRight     =>  FRAC_PI_2 - FRAC_2_PI,
            Self::BotHalfRight =>  FRAC_PI_2,
            Self::BotHalfLeft  =>  FRAC_PI_2 + FRAC_2_PI,
            Self::BotLeft      =>  (PI - FRAC_2_PI)
        }
    }

    pub fn find_part_for_angle(angle: f32) -> Self {
        Self::iter()
            .find(|hex_part| angle < hex_part.arc_end())
            .unwrap_or(Self::Left)
    }
}
