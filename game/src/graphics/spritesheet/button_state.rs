use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum ButtonState {
    Base,
    Pressed,
    Disabled,
    Hovered,
}

impl super::AnimationType for ButtonState {
    const DEF_TYPE: u32 = 71;

    fn container_index(&self) -> u32 {
        match self {
            Self::Base => 0,
            Self::Pressed => 1,
            Self::Disabled => 2,
            Self::Hovered => 3,
        }
    }

    fn array_index(&self) -> usize {
        *self as usize
    }
}
