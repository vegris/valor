use std::{collections::HashMap, marker::PhantomData};

use sdl2::pixels::{Color, Palette};
use strum::{EnumCount, IntoEnumIterator};

use formats::{DefContainer, DefSprite};

pub mod creature;
pub mod hero;
mod sprite;

use sprite::Sprite;

type AnimationBlock = Box<[usize]>;

pub struct Spritesheet<A: AnimationType> {
    colors: Box<[Color]>,
    sprites: Box<[Sprite]>,
    blocks: Box<[Option<AnimationBlock>]>,
    animation_type: PhantomData<A>,
}

pub trait AnimationType: EnumCount + IntoEnumIterator {
    const DEF_TYPE: u32;

    fn index(&self) -> u32;
}

impl<A: AnimationType> Spritesheet<A> {
    pub fn from_def(def_container: DefContainer) -> Self {
        let DefContainer {
            type_,
            mut colors,
            blocks2names,
            names2sprites,
        } = def_container;

        assert!(type_ == A::DEF_TYPE);

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
            .map(Sprite::from_def)
            .collect::<Box<[Sprite]>>();
        sprites
            .iter_mut()
            .for_each(|sprite| sprite.surface.set_palette(&palette).unwrap());

        // Блоки анимаций - последовательности индексов спрайтов
        let mut blocks = Vec::with_capacity(A::COUNT);
        blocks.resize(A::COUNT, None);

        for (index, animation_type) in A::iter().enumerate() {
            if let Some(block) = blocks2names.get(&animation_type.index()) {
                let block = block
                    .iter()
                    .map(|sprite_name| names2indexes[sprite_name])
                    .collect::<AnimationBlock>();

                blocks[index] = Some(block);
            }
        }

        let blocks = blocks.into_boxed_slice();

        Spritesheet {
            colors,
            sprites,
            blocks,
            animation_type: PhantomData,
        }
    }

    pub fn get_sprite(&self, block_index: usize, progress: f32) -> Option<&Sprite> {
        assert!((0.0..=1.0).contains(&progress));
        self.blocks[block_index]
            .as_ref()
            .map(|block| &self.sprites[sprite_index(block, progress)])
    }
}

fn sprite_index(block: &AnimationBlock, progress: f32) -> usize {
    block[((block.len() - 1) as f32 * progress).round() as usize]
}
