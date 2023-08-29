use sdl2::mouse::Cursor as SDLCursor;

use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

use crate::grid::AttackDirection;
use crate::registry::spritesheets::{ContainerType, SpriteGroupType};
use crate::registry::ResourceRegistry;

#[derive(Clone, Copy, EnumCount, EnumIter, Debug)]
pub enum Cursor {
    Forbidden = 0,
    Run = 1,
    Fly = 2,
    Arrow = 3,
    Hero = 4,
    Question = 5,
    Pointer = 6,
    AttackDownLeft = 7,
    AttackLeft = 8,
    AttackUpLeft = 9,
    AttackUpRight = 10,
    AttackRight = 11,
    AttackDownRight = 12,
    AttackDown = 13,
    AttackUp = 14,
    ArrowBroken = 15,
    Catapult = 16,
    Heal = 17,
    Sacrifice = 18,
    Teleport = 19,
}

impl Cursor {
    const fn pointer_offset(self) -> (i32, i32) {
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

    pub fn from_attack_direction(attack_direction: AttackDirection) -> Self {
        match attack_direction {
            AttackDirection::Left => Self::AttackLeft,
            AttackDirection::TopLeft => Self::AttackUpLeft,
            AttackDirection::Top => Self::AttackUp,
            AttackDirection::TopRight => Self::AttackUpRight,
            AttackDirection::Right => Self::AttackRight,
            AttackDirection::BottomRight => Self::AttackDownRight,
            AttackDirection::Bottom => Self::AttackDown,
            AttackDirection::BottomLeft => Self::AttackDownLeft,
        }
    }
}

impl ContainerType for Cursor {
    const CONTAINER_TYPE: u32 = 70;
}

impl SpriteGroupType for Cursor {
    fn group_index(&self) -> usize {
        *self as usize
    }
}

const CONTAINTER_FILENAME: &str = "CRCOMBAT.def";

pub struct Cursors(Box<[SDLCursor]>);

impl Cursors {
    pub fn load(rr: &mut ResourceRegistry) -> Self {
        let sprite_group = rr.load_sprite_group::<Cursor>(CONTAINTER_FILENAME);

        let cursors = sprite_group
            .into_sprites()
            .into_vec() // Boxed slice problems...
            .into_iter()
            .zip(Cursor::iter())
            .map(|(sprite, cursor)| {
                let (off_x, off_y) = cursor.pointer_offset();
                SDLCursor::from_surface(sprite.surface, off_x, off_y).unwrap()
            })
            .collect();

        Self(cursors)
    }

    pub fn get(&self, cursor: Cursor) -> &SDLCursor {
        &self.0[cursor as usize]
    }
}
