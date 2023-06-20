use std::collections::HashMap;

extern crate sdl2;
use sdl2::pixels::{Color, Palette};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;

extern crate strum_macros;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use formats::{DefContainer, DefSprite};

use crate::battlestate::Side;

// Номера повторяют номера в реальном Def файле
#[derive(Debug, Clone, Copy, PartialEq, EnumCountMacro, EnumIter)]
pub enum AnimationType {
    Moving,
    MouseOver,
    Standing,
    GettingHit,
    Defend,
    Death,
    UnusedDeath,
    TurnLeft,
    TurnRight,
    AttackUp,
    AttackStraight,
    AttackDown,
    ShootUp,
    ShootStraight,
    ShootDown,
    TwoHexAttackUp,
    TwoHexAttackStraight,
    TwoHexAttackDown,
    StartMoving,
    StopMoving,
}

impl AnimationType {
    const fn def_container_index(self) -> u32 {
        match self {
            AnimationType::Moving => 0,
            AnimationType::MouseOver => 1,
            AnimationType::Standing => 2,
            AnimationType::GettingHit => 3,
            AnimationType::Defend => 4,
            AnimationType::Death => 5,
            AnimationType::UnusedDeath => 6,
            AnimationType::TurnLeft => 7,
            AnimationType::TurnRight => 8,
            // Дублируются
            // TurnLeft_DBL = 9,
            // TurnRight_DBL = 10,
            AnimationType::AttackUp => 11,
            AnimationType::AttackStraight => 12,
            AnimationType::AttackDown => 13,
            AnimationType::ShootUp => 14,
            AnimationType::ShootStraight => 15,
            AnimationType::ShootDown => 16,
            AnimationType::TwoHexAttackUp => 17,
            AnimationType::TwoHexAttackStraight => 18,
            AnimationType::TwoHexAttackDown => 19,
            AnimationType::StartMoving => 20,
            AnimationType::StopMoving => 21,
        }
    }
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

    pub fn draw_rect(&self, center: Point, side: Side) -> Rect {
        const FULL_WIDTH: u32 = 450;
        const FULL_HEIGHT: u32 = 400;

        // Поправка чтобы спрайт существа ровно располагался на спрайте клетки
        const X_CORRECTION: i32 = 30;
        const Y_CORRECTION: i32 = -50;

        let Self {
            left_margin,
            top_margin,
            width,
            height,
            ..
        } = *self;

        let full_rect = Rect::from_center(center, FULL_WIDTH, FULL_HEIGHT);
        let (reference_point, x_offset) = match side {
            Side::Attacker => (full_rect.top_left(), left_margin as i32 + X_CORRECTION),
            Side::Defender => (
                full_rect.top_right(),
                -((left_margin + width) as i32 + X_CORRECTION),
            ),
        };

        let top_left = reference_point.offset(x_offset, top_margin as i32 + Y_CORRECTION);
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
    const CREATURE_DEF_TYPE: u32 = 66;

    pub fn from_def_container(def_container: DefContainer) -> Self {
        let DefContainer {
            type_,
            mut colors,
            blocks2names,
            names2sprites,
        } = def_container;

        assert!(type_ == Self::CREATURE_DEF_TYPE);

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

        for animation_type in AnimationType::iter() {
            if let Some(block) = blocks2names.get(&animation_type.def_container_index()) {
                let block = block
                    .iter()
                    .map(|sprite_name| names2indexes[sprite_name])
                    .collect::<AnimationBlock>();
                blocks[animation_type as usize] = Some(block);
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
    block[(block.len() as f32 * progress) as usize]
}
