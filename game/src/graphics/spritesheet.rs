use std::collections::HashMap;

use formats::{DefContainer, DefSprite};
use sdl2::pixels::Color;
use sdl2::{pixels::Palette, surface::Surface};
use strum::{EnumCount, IntoEnumIterator};

pub mod creature;
pub mod hero;

type AnimationBlock = Box<[usize]>;

pub struct Container {
    colors: Box<[Color]>,
    sprites: Box<[Sprite]>,
    blocks: Box<[Option<AnimationBlock>]>,
}

pub struct Sprite {
    width: u32,
    height: u32,
    left_margin: u32,
    top_margin: u32,
    surface: Surface<'static>,
}

impl Sprite {
    fn from_def_sprite(def_sprite: DefSprite) -> Self {
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

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}

pub trait Spritesheet {
    type A: AnimationT;

    fn from_container(container: DefContainer) -> Self
    where
        Self: Sized,
    {
        let DefContainer {
            type_,
            mut colors,
            blocks2names,
            names2sprites,
        } = container;

        assert!(type_ == Self::A::DEF_TYPE);

        // Применяем прозрачность
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
        let (names, def_sprites): (Vec<String>, Vec<DefSprite>) = names2sprites.into_iter().unzip();
        let names2indexes = names
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<String, usize>>();
        let mut sprites = def_sprites
            .into_iter()
            .map(Sprite::from_def_sprite)
            .collect::<Box<[Sprite]>>();
        sprites
            .iter_mut()
            .for_each(|sprite| sprite.surface.set_palette(&palette).unwrap());

        // Блоки анимаций - последовательности индексов спрайтов
        let mut blocks = Vec::with_capacity(Self::A::COUNT);
        blocks.resize(Self::A::COUNT, None);

        for (index, animation_type) in Self::A::iter().enumerate() {
            if let Some(block) = blocks2names.get(&animation_type.index()) {
                let block = block
                    .iter()
                    .map(|sprite_name| names2indexes[sprite_name])
                    .collect::<AnimationBlock>();

                blocks[index] = Some(block);
            }
        }

        let blocks = blocks.into_boxed_slice();

        let container = Container {
            colors,
            sprites,
            blocks,
        };
        Self::to_self(container)
    }

    fn to_self(container: Container) -> Self
    where
        Self: Sized;

    fn container(&self) -> &Container;

    fn get_sprite(&self, animation: Self::A, progress: f32) -> Option<&Sprite> {
        assert!((0.0..=1.0).contains(&progress));
        self.container().blocks[animation.value()]
            .as_ref()
            .map(|block| &self.container().sprites[sprite_index(block, progress)])
    }
}

fn sprite_index(block: &AnimationBlock, progress: f32) -> usize {
    block[(block.len() as f32 * progress) as usize]
}

pub trait AnimationT: EnumCount + IntoEnumIterator {
    const DEF_TYPE: u32;

    fn index(&self) -> u32;

    fn value(&self) -> usize;
}
