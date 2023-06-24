use formats::DefContainer;
use sdl2::pixels::{Color, Palette};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;
use strum_macros::{EnumCount, EnumIter};

use crate::battlestate::Side;

use super::sprite::Sprite;
use super::Container;

#[derive(Clone, Copy, EnumCount, EnumIter, PartialEq)]
pub enum AnimationType {
    Moving,
    MouseOver,
    Standing,
    GettingHit,
    Defend,
    Death,
    UnusedDeath,
    TurnLeft,
    TurnRight,
    AttackUp,
    AttackStraight,
    AttackDown,
    ShootUp,
    ShootStraight,
    ShootDown,
    TwoHexAttackUp,
    TwoHexAttackStraight,
    TwoHexAttackDown,
    StartMoving,
    StopMoving,
}

impl super::AnimationType for AnimationType {
    const DEF_TYPE: u32 = 66;

    fn index(&self) -> u32 {
        match self {
            Self::Moving => 0,
            Self::MouseOver => 1,
            Self::Standing => 2,
            Self::GettingHit => 3,
            Self::Defend => 4,
            Self::Death => 5,
            Self::UnusedDeath => 6,
            Self::TurnLeft => 7,
            Self::TurnRight => 8,
            // Дублируются
            // TurnLeft_DBL = 9,
            // TurnRight_DBL = 10,
            Self::AttackUp => 11,
            Self::AttackStraight => 12,
            Self::AttackDown => 13,
            Self::ShootUp => 14,
            Self::ShootStraight => 15,
            Self::ShootDown => 16,
            Self::TwoHexAttackUp => 17,
            Self::TwoHexAttackStraight => 18,
            Self::TwoHexAttackDown => 19,
            Self::StartMoving => 20,
            Self::StopMoving => 21,
        }
    }
}

pub struct Creature(Container);

impl Creature {
    pub fn from_def(def: DefContainer) -> Self {
        Self(super::Container::from_def::<AnimationType>(def))
    }

    pub fn get_sprite(&self, animation_type: AnimationType, progress: f32) -> Option<&Sprite> {
        self.0.get_sprite(animation_type as usize, progress)
    }
}

pub fn with_selection(sprite: &Sprite, spritesheet: &Creature) -> Surface<'static> {
    let mut surface = sprite
        .surface
        .convert(&sprite.surface.pixel_format())
        .unwrap();

    let mut colors = spritesheet.0.colors.to_vec();
    colors[5] = Color::YELLOW;
    let palette = Palette::with_colors(&colors).unwrap();

    surface.set_palette(&palette).unwrap();

    surface
}

pub fn draw_rect(sprite: &Sprite, center: Point, side: Side) -> Rect {
    const FULL_WIDTH: u32 = 450;
    const FULL_HEIGHT: u32 = 400;

    // Поправка чтобы спрайт существа ровно располагался на спрайте клетки
    const X_CORRECTION: i32 = 30;
    const Y_CORRECTION: i32 = -50;

    let Sprite {
        left_margin,
        top_margin,
        width,
        height,
        ..
    } = *sprite;

    let full_rect = Rect::from_center(center, FULL_WIDTH, FULL_HEIGHT);
    let (reference_point, x_offset) = match side {
        Side::Attacker => (full_rect.top_left(), left_margin as i32 + X_CORRECTION),
        Side::Defender => (
            full_rect.top_right(),
            -((left_margin + width) as i32 + X_CORRECTION),
        ),
    };

    let top_left = reference_point.offset(x_offset, top_margin as i32 + Y_CORRECTION);
    Rect::new(top_left.x(), top_left.y(), width, height)
}
