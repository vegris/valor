use std::marker::PhantomData;

use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;
use strum::EnumCount;

use formats::def;

pub trait SpriteGroupType: ContainerType + EnumCount + GroupIndex {}

pub trait ContainerType {
    const CONTAINER_TYPE: u32;
}

pub trait GroupIndex {
    fn index(&self) -> usize;
}

pub struct SpriteGroup<G: SpriteGroupType> {
    sprites: Box<[Sprite]>,
    group: PhantomData<G>,
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub surface: Surface<'static>,
}

impl<G: SpriteGroupType> SpriteGroup<G> {
    pub fn from_bytes(bytes: Box<[u8]>) -> Self {
        let mut raw = def::Container::from_bytes(&bytes);

        assert!(raw.type_ == G::CONTAINER_TYPE);
        assert!(raw.blocks2names.len() == 1);

        let block_names = raw.blocks2names.into_values().next().unwrap();

        assert!(block_names.len() == G::COUNT);

        let mut colors: Box<[Color]> = raw
            .colors
            .iter()
            .map(|c| Color::RGB(c.red, c.green, c.blue))
            .collect();
        colors[0] = Color::RGBA(0, 0, 0, 0);
        colors[1] = Color::RGBA(0, 0, 0, 32);
        let palette = Palette::with_colors(&colors).unwrap();

        let mut sprites = Vec::with_capacity(G::COUNT);
        for name in block_names.iter() {
            let raw_sprite = raw.names2sprites.remove(name).unwrap();
            let mut sprite = Sprite::from_raw(raw_sprite);
            sprite.surface.set_palette(&palette).unwrap();
            sprites.push(sprite);
        }

        Self {
            sprites: sprites.into_boxed_slice(),
            group: PhantomData,
        }
    }

    pub fn get(&self, index: G) -> &Sprite {
        &self.sprites[index.index()]
    }
}

impl Sprite {
    fn from_raw(mut raw: def::Sprite) -> Self {
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
