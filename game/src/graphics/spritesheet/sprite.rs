use sdl2::{pixels::Color, surface::Surface};

use formats::DefSprite;

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub surface: Surface<'static>,
}

impl Sprite {
    pub fn from_def(def_sprite: DefSprite) -> Self {
        let DefSprite {
            width,
            height,
            left_margin,
            top_margin,
            mut surface,
            ..
        } = def_sprite;

        surface.set_color_key(true, Color::BLACK).unwrap();

        Self {
            width,
            height,
            left_margin,
            top_margin,
            surface,
        }
    }
}
