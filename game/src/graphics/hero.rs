use std::collections::HashMap;

extern crate sdl2;
use sdl2::pixels::{Color, Palette};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;

extern crate strum_macros;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use formats::{DefContainer, DefSprite};

// Номера повторяют номера в реальном Def файле
#[derive(Debug, Clone, Copy, PartialEq, EnumCountMacro, EnumIter)]
pub enum AnimationType {
    Still = 0,
    Idle = 1,
    Facepalm = 2,
    Happy = 3,
    Casting = 4,
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

    fn apply_palette(&mut self, palette: &Palette) {
        self.surface.set_palette(palette).unwrap();
    }

    pub fn with_selection(&self, spritesheet: &Spritesheet) -> Surface<'static> {
        let mut surface = self.surface.convert(&self.surface.pixel_format()).unwrap();

        let mut colors = spritesheet.colors.to_vec();
        colors[5] = Color::YELLOW;
        let palette = Palette::with_colors(&colors).unwrap();

        surface.set_palette(&palette).unwrap();

        surface
    }

    pub fn draw_rect(&self, center: Point) -> Rect {
        const FULL_WIDTH: u32 = 150;
        const FULL_HEIGHT: u32 = 175;

        let Self {
            left_margin,
            top_margin,
            width,
            height,
            ..
        } = *self;

        let full_rect = Rect::from_center(center, FULL_WIDTH, FULL_HEIGHT);
        let (reference_point, x_offset) = (full_rect.top_left(), left_margin as i32);

        let top_left = reference_point.offset(x_offset, top_margin as i32);
        Rect::new(top_left.x(), top_left.y(), width, height)
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}

type AnimationBlock = Box<[usize]>;

pub struct Spritesheet {
    colors: Box<[Color]>,
    sprites: Box<[Sprite]>,
    blocks: [Option<AnimationBlock>; AnimationType::COUNT],
}

impl Spritesheet {
    const DEF_TYPE: u32 = 73;

    pub fn from_def_container(def_container: DefContainer) -> Self {
        let DefContainer {
            type_,
            mut colors,
            blocks2names,
            names2sprites,
        } = def_container;

        assert!(type_ == Self::DEF_TYPE);

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
            .for_each(|sprite| sprite.apply_palette(&palette));

        // Блоки анимаций - последовательности индексов спрайтов
        const NONE: Option<AnimationBlock> = None;
        let mut blocks = [NONE; AnimationType::COUNT];

        for (block_index, _) in AnimationType::iter().enumerate() {
            if let Some(block) = blocks2names.get(&(block_index as u32)) {
                let block = block
                    .iter()
                    .map(|sprite_name| names2indexes[sprite_name])
                    .collect::<AnimationBlock>();
                blocks[block_index] = Some(block);
            }
        }

        Self {
            colors,
            sprites,
            blocks,
        }
    }

    pub fn get_sprite(&self, animation: AnimationType, progress: f32) -> Option<&Sprite> {
        assert!((0.0..=1.0).contains(&progress));
        self.blocks[animation as usize]
            .as_ref()
            .map(|block| &self.sprites[sprite_index(block, progress)])
    }
}

fn sprite_index(block: &AnimationBlock, progress: f32) -> usize {
    block[((block.len() - 1) as f32 * progress) as usize]
}
