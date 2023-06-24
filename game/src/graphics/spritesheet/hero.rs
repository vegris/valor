use formats::DefContainer;
use sdl2::rect::{Point, Rect};
use strum_macros::{EnumCount, EnumIter};

use super::sprite::Sprite;
use super::Container;

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum AnimationType {
    Idle,
    Facepalm,
    Happy,
    Casting,
}

impl super::AnimationType for AnimationType {
    const DEF_TYPE: u32 = 73;

    fn index(&self) -> u32 {
        match self {
            AnimationType::Idle => 1,
            AnimationType::Facepalm => 2,
            AnimationType::Happy => 3,
            AnimationType::Casting => 4,
        }
    }
}

pub struct Hero(Container);

impl Hero {
    pub fn from_def(def: DefContainer) -> Self {
        Self(super::Container::from_def::<AnimationType>(def))
    }

    pub fn get_sprite(&self, animation_type: AnimationType, progress: f32) -> Option<&Sprite> {
        self.0.get_sprite(animation_type as usize, progress)
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
