use std::collections::HashMap;
use std::marker::PhantomData;

use common::EnumIndex;
use formats::def;
use gamedata::traits::{AnimationGroupT, ContainerType, SpriteGroupT};
use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;
use strum::{EnumCount, IntoEnumIterator};

// def контейнер это основной формат хранения спрайтов в HoMM 3
// он может использоваться как для кодирования последовательностей анимаций
// так и просто как контейнер для различных тематически сходных изображений
// структурно он всегда один и тот же, но семантически мы хотим использовать его по разному
//
// Этот модуль определяет трейты и структуры для различных типов использования def контейнера:
//
// **SpriteGroup** и **SpriteGroupType** используются когда контейнер состоит из одного блока,
// представляющего набор связанных изображений - курсоры, иконки заклинаний, иконки интерфейса
// Они предоставляют возможность с помощью перечисления адресовать отдельные изображения внутри
// контейнера
//
// **AnimationGroup** и **AnimationGroupType** используются когда контейнер состоит из нескольких блоков
// (последовательностей изображений), где каждая последовательность является анимацией.
// Они предоставляют возможность с помощью перечисления адресовать блоки внутри контейнера и
// отдельные кадры внутри блоков с помощью индексов
//
// **SingleAnimation** используется в случаях когда **AnimationGroup** состоит из одной анимации

#[derive(Clone, Copy)]
struct SingleAnimation<const T: u32>;

impl<const T: u32> ContainerType for SingleAnimation<T> {
    const CONTAINER_TYPE: u32 = T;
}

impl<const T: u32> EnumCount for SingleAnimation<T> {
    const COUNT: usize = 1;
}

impl<const T: u32> EnumIndex for SingleAnimation<T> {
    fn into_index(self) -> usize {
        0
    }
}

impl<const T: u32> IntoEnumIterator for SingleAnimation<T> {
    type Iterator = std::array::IntoIter<Self, 1>;

    fn iter() -> Self::Iterator {
        [SingleAnimation].into_iter()
    }
}

impl<const T: u32> AnimationGroupT for SingleAnimation<T> {
    fn container_index(self) -> u32 {
        0
    }
}

pub struct SpriteGroup<G: SpriteGroupT> {
    sprites: Box<[Sprite]>,
    group: PhantomData<G>,
}

type AnimationBlock = Box<[usize]>;
pub struct AnimationGroup<S: AnimationGroupT> {
    colors: Box<[Color]>,
    sprites: Box<[Sprite]>,
    blocks: Box<[Option<AnimationBlock>]>,
    spritesheet: PhantomData<S>,
}

pub struct SpriteSheetSingle(Box<[Sprite]>);

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub surface: Surface<'static>,
}

type ColorUpdate = (usize, u8);

impl<G: SpriteGroupT> SpriteGroup<G> {
    const COLOR_UPDATE_LIST: [ColorUpdate; 2] = [(0, 0), (1, 32)];

    pub fn from_def(mut raw: def::Container) -> Self {
        assert!(raw.type_ == G::CONTAINER_TYPE);
        assert!(raw.blocks2names.len() == 1);

        let block_names = raw.blocks2names.into_values().next().unwrap();

        assert!(block_names.len() == G::COUNT);

        let colors = make_colors(&raw.colors, &Self::COLOR_UPDATE_LIST);
        let palette = Palette::with_colors(&colors).unwrap();

        let sprites: Box<[_]> = block_names
            .iter()
            .map(|name| {
                let raw_sprite = raw.names2sprites.remove(name).unwrap();
                let mut sprite = Sprite::from_raw(raw_sprite);
                sprite.surface.set_palette(&palette).unwrap();
                sprite
            })
            .collect();

        Self {
            sprites,
            group: PhantomData,
        }
    }

    pub fn get(&self, index: G) -> &Sprite {
        &self.sprites[index.into_index()]
    }

    pub fn into_sprites(self) -> Box<[Sprite]> {
        self.sprites
    }
}

impl<S: AnimationGroupT> AnimationGroup<S> {
    const COLOR_UPDATE_LIST: [(usize, u8); 8] = [
        (0, 0),
        (1, 32),
        (2, 64),
        (3, 128),
        (4, 128),
        (5, 0),
        (6, 128),
        (7, 64),
    ];

    pub fn from_def(raw: def::Container) -> Self {
        assert!(raw.type_ == S::CONTAINER_TYPE);

        let colors = make_colors(&raw.colors, &Self::COLOR_UPDATE_LIST);
        let palette = Palette::with_colors(&colors).unwrap();

        // Вместо HashMap с поиском по строке
        // переходим к поиску по индексу в массиве
        let (names, def_sprites): (Vec<String>, Vec<def::Sprite>) =
            raw.names2sprites.into_iter().unzip();

        let names2indexes = names
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<String, usize>>();

        let sprites = def_sprites
            .into_iter()
            .map(|def_sprite| {
                let mut sprite = Sprite::from_raw(def_sprite);
                sprite.surface.set_palette(&palette).unwrap();
                sprite
            })
            .collect::<Box<[Sprite]>>();

        // Блоки анимаций - последовательности индексов спрайтов
        let mut blocks = Vec::new();
        blocks.resize(S::COUNT, None);

        for animation_type in S::iter() {
            if let Some(block) = raw.blocks2names.get(&animation_type.container_index()) {
                let block = block
                    .iter()
                    .map(|sprite_name| names2indexes[sprite_name])
                    .collect::<AnimationBlock>();

                blocks[animation_type.into_index()] = Some(block);
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

    pub fn has_animation(&self, animation_type: S) -> bool {
        self.get_block(animation_type).is_some()
    }

    fn get_block(&self, animation_type: S) -> Option<&AnimationBlock> {
        self.blocks[animation_type.into_index()].as_ref()
    }
}

impl SpriteSheetSingle {
    // TODO: use ContainerType instead of constant
    pub fn from_def<const T: u32>(raw: def::Container) -> Self {
        let spritesheet: AnimationGroup<SingleAnimation<T>> = AnimationGroup::from_def(raw);

        let block = spritesheet.blocks.to_vec().remove(0).unwrap().into_vec();

        let mut sprite_vector: Vec<_> = spritesheet
            .sprites
            .into_vec()
            .into_iter()
            .map(Option::Some)
            .collect();

        let mut sprites = Vec::with_capacity(block.len());

        for index in block.into_iter() {
            let sprite = sprite_vector[index].take().unwrap();
            sprites.push(sprite);
        }

        Self(sprites.into_boxed_slice())
    }

    pub fn frames_count(&self) -> usize {
        self.0.len()
    }

    pub fn get_frame(&self, frame_index: usize) -> Option<&Sprite> {
        self.0.get(frame_index)
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

fn make_colors(colors: &[formats::Color], color_update_list: &[ColorUpdate]) -> Box<[Color]> {
    let mut colors: Box<[Color]> = colors
        .iter()
        .map(|c| Color::RGB(c.red, c.green, c.blue))
        .collect();

    for color_update in color_update_list.iter() {
        colors[color_update.0] = Color::RGBA(0, 0, 0, color_update.1);
    }

    colors
}
