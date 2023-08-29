use std::collections::HashMap;
use std::marker::PhantomData;

use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;
use strum::{EnumCount, IntoEnumIterator};

use formats::def;

pub trait SpriteGroupType: ContainerType + EnumCount {
    fn group_index(&self) -> usize;
}
pub trait SpriteSheetType: ContainerType + EnumCount + IntoEnumIterator {
    fn block_index(&self) -> usize;
    fn container_index(&self) -> u32;
}

pub trait ContainerType {
    const CONTAINER_TYPE: u32;
}

pub struct SpriteGroup<G: SpriteGroupType> {
    sprites: Box<[Sprite]>,
    group: PhantomData<G>,
}

type AnimationBlock = Box<[usize]>;
pub struct SpriteSheet<S: SpriteSheetType> {
    colors: Box<[Color]>,
    sprites: Box<[Sprite]>,
    blocks: Box<[Option<AnimationBlock>]>,
    spritesheet: PhantomData<S>,
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
        &self.sprites[index.group_index()]
    }

    pub fn into_sprites(self) -> Box<[Sprite]> {
        self.sprites
    }
}

impl<S: SpriteSheetType> SpriteSheet<S> {
    pub fn from_bytes(bytes: Box<[u8]>) -> Self {
        let raw = def::Container::from_bytes(&bytes);

        assert!(raw.type_ == S::CONTAINER_TYPE);

        let mut colors: Box<[Color]> = raw
            .colors
            .iter()
            .map(|c| Color::RGB(c.red, c.green, c.blue))
            .collect();
        colors[0] = Color::RGBA(0, 0, 0, 0);
        colors[1] = Color::RGBA(0, 0, 0, 32);
        colors[2] = Color::RGBA(0, 0, 0, 64);
        colors[3] = Color::RGBA(0, 0, 0, 128);
        colors[4] = Color::RGBA(0, 0, 0, 128);
        colors[5] = Color::RGBA(0, 0, 0, 0);
        colors[6] = Color::RGBA(0, 0, 0, 128);
        colors[7] = Color::RGBA(0, 0, 0, 64);
        let palette = Palette::with_colors(&colors).unwrap();

        // Вместо мапы имена => спрайты находим нужный спрайт по его индексу в массиве спрайтов
        let (names, def_sprites): (Vec<String>, Vec<def::Sprite>) =
            raw.names2sprites.into_iter().unzip();
        let names2indexes = names
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<String, usize>>();
        let mut sprites = def_sprites
            .into_iter()
            .map(Sprite::from_raw)
            .collect::<Box<[Sprite]>>();

        for sprite in sprites.iter_mut() {
            sprite.surface.set_palette(&palette).unwrap();
        }

        // Блоки анимаций - последовательности индексов спрайтов
        let mut blocks = Vec::with_capacity(S::COUNT);
        blocks.resize(S::COUNT, None);

        for (index, animation_type) in S::iter().enumerate() {
            if let Some(block) = raw.blocks2names.get(&animation_type.container_index()) {
                let block = block
                    .iter()
                    .map(|sprite_name| names2indexes[sprite_name])
                    .collect::<AnimationBlock>();

                blocks[index] = Some(block);
            }
        }

        Self {
            colors,
            sprites,
            blocks: blocks.into_boxed_slice(),
            spritesheet: PhantomData,
        }
    }

    pub fn get_sprite(&self, animation_type: S, frame_index: usize) -> Option<&Sprite> {
        self.get_block(animation_type).map(|block| {
            let sprite_index = block[frame_index];
            &self.sprites[sprite_index]
        })
    }

    pub fn frames_count(&self, animation_type: S) -> Option<usize> {
        self.get_block(animation_type).map(|block| block.len())
    }

    pub fn colors(&self) -> &[Color] {
        &self.colors
    }

    fn get_block(&self, animation_type: S) -> Option<&AnimationBlock> {
        self.blocks[animation_type.block_index()].as_ref()
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

        let mut static_surface = surface.convert_format(surface.pixel_format_enum()).unwrap();
        static_surface.set_color_key(true, Color::BLACK).unwrap();

        Self {
            width: raw.width,
            height: raw.height,
            left_margin: raw.left_margin,
            top_margin: raw.top_margin,
            surface: static_surface,
        }
    }
}
