extern crate sdl2;
use sdl2::mouse::Cursor as SDLCursor;
use sdl2::pixels::{Color, Palette};

extern crate strum_macros;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::grid::AttackDirection;
use crate::registry::def::Container;
use crate::registry::ResourceRegistry;

#[derive(Clone, Copy, EnumIter, Debug)]
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

const CONTAINTER_FILENAME: &str = "CRCOMBAT.def";

pub struct Cursors(Box<[SDLCursor]>);

impl Cursors {
    pub fn load(rr: &mut ResourceRegistry) -> Self {
        let def_container = rr.load_def(CONTAINTER_FILENAME);

        let Container {
            mut names2sprites,
            blocks2names,
            mut colors,
            ..
        } = def_container;

        // Применяем прозрачность
        colors[0] = Color::RGBA(0, 0, 0, 0);
        colors[1] = Color::RGBA(0, 0, 0, 32);
        let palette = Palette::with_colors(&colors).unwrap();

        let block = blocks2names.get(&0).unwrap();

        let cursors = Iterator::zip(block.iter(), Cursor::iter())
            .map(|(name, cursor)| {
                let sprite = names2sprites.remove(name).unwrap();
                let mut surface = sprite.surface;
                surface.set_palette(&palette).unwrap();
                let (off_x, off_y) = cursor.pointer_offset();
                SDLCursor::from_surface(surface, off_x, off_y).unwrap()
            })
            .collect::<Box<[SDLCursor]>>();

        Self(cursors)
    }

    pub fn get(&self, cursor: Cursor) -> &SDLCursor {
        &self.0[cursor as usize]
    }
}
