use sdl2::rect::{Point, Rect};
use strum_macros::{EnumCount, EnumIter};

use super::{AnimationT, Container, Sprite, Spritesheet};

#[derive(Debug, Clone, Copy, EnumCount, EnumIter)]
pub enum AnimationType {
    Idle,
    Facepalm,
    Happy,
    Casting,
}

impl AnimationT for AnimationType {
    const DEF_TYPE: u32 = 73;

    fn index(&self) -> u32 {
        match self {
            AnimationType::Idle => 1,
            AnimationType::Facepalm => 2,
            AnimationType::Happy => 3,
            AnimationType::Casting => 4,
        }
    }

    fn value(&self) -> usize {
        *self as usize
    }
}

pub struct HeroSpritesheet(Container);

impl Spritesheet for HeroSpritesheet {
    type A = AnimationType;

    fn to_self(container: Container) -> Self
    where
        Self: Sized,
    {
        Self(container)
    }

    fn container(&self) -> &Container {
        &self.0
    }
}

pub fn draw_rect(sprite: &Sprite, center: Point) -> Rect {
    const FULL_WIDTH: u32 = 150;
    const FULL_HEIGHT: u32 = 175;

    let Sprite {
        left_margin,
        top_margin,
        width,
        height,
        ..
    } = *sprite;

    let full_rect = Rect::from_center(center, FULL_WIDTH, FULL_HEIGHT);
    let (reference_point, x_offset) = (full_rect.top_left(), left_margin as i32);

    let top_left = reference_point.offset(x_offset, top_margin as i32);
    Rect::new(top_left.x(), top_left.y(), width, height)
}
