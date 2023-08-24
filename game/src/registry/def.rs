use std::collections::HashMap;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;

use formats::def::container::Container as RawContainer;
use formats::def::sprite::Sprite as RawSprite;

pub struct Container {
    pub type_: u32,
    pub colors: Box<[Color]>,
    pub names2sprites: HashMap<String, Sprite>,
    pub blocks2names: HashMap<u32, Box<[String]>>,
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub surface: Surface<'static>,
}

impl Container {
    pub fn from_raw(raw: RawContainer) -> Self {
        let names2sprites = raw
            .names2sprites
            .into_iter()
            .map(|(name, sprite)| (name, Sprite::from_raw(sprite)))
            .collect();

        let colors = raw
            .colors
            .iter()
            .map(|c| Color::RGB(c.red, c.green, c.blue))
            .collect();

        Self {
            type_: raw.type_,
            colors,
            blocks2names: raw.blocks2names,
            names2sprites,
        }
    }
}

impl Sprite {
    fn from_raw(mut raw: RawSprite) -> Self {
        let surface = Surface::from_data(
            &mut raw.pixels,
            raw.width,
            raw.height,
            raw.width,
            PixelFormatEnum::Index8,
        )
        .unwrap();
        let static_surface = surface.convert_format(surface.pixel_format_enum()).unwrap();

        Self {
            width: raw.width,
            height: raw.height,
            left_margin: raw.left_margin,
            top_margin: raw.top_margin,
            surface: static_surface,
        }
    }
}
