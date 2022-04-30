use std::collections::HashMap;

extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, Palette};
use sdl2::rect::{Point, Rect};

extern crate strum_macros;
use strum::{IntoEnumIterator, EnumCount};
use strum_macros::{EnumIter, EnumCount as EnumCountMacro};

use formats::{DefSprite, DefContainer};

use crate::battlestate::Side;

// Номера повторяют номера в реальном Def файле
#[derive(Debug, Clone, Copy, PartialEq, EnumCountMacro, EnumIter)]
#[allow(non_camel_case_types)]
pub enum AnimationType {
    Moving = 0,
    MouseOver = 1,
    Standing = 2,
    GettingHit = 3,
    Defend = 4,
    Death = 5,
    UnusedDeath = 6,
    TurnLeft = 7,
    TurnRight = 8,
    // Дублируются
    TurnLeft_DBL = 9,
    TurnRight_DBL = 10,
    AttackUp = 11,
    AttackStraight = 12,
    AttackDown = 13,
    ShootUp = 14,
    ShootStraight = 15,
    ShootDown = 16,
    TwoHexAttackUp = 17,
    TwoHexAttackStraight = 18,
    TwoHexAttackDown = 19,
    StartMoving = 20,
    StopMoving = 21
}

pub struct CreatureSprite {
    width: u32,
    height: u32,
    left_margin: u32,
    top_margin: u32,
    surface: Surface<'static>
}

impl CreatureSprite {
    fn from_def_sprite(def_sprite: DefSprite) -> Self {
        let DefSprite { width, height, left_margin, top_margin, mut surface, .. } = def_sprite;

        surface.set_color_key(true, Color::BLACK).unwrap();

        Self { width, height, left_margin, top_margin, surface }
    }

    fn apply_palette(&mut self, palette: &Palette) {
        self.surface.set_palette(palette).unwrap();
    }

    pub fn turn_selection(&mut self, colors: &mut Box<[Color]>, on: bool) {
        colors[5] = if on { Color::YELLOW } else { Color::RGBA(0, 0, 0, 0) };
        let palette = Palette::with_colors(colors).unwrap();
        self.apply_palette(&palette);
    }

    pub fn draw_rect(&self, center: Point, side: Side) -> Rect {
        const FULL_WIDTH: u32 = 450;
        const FULL_HEIGHT: u32 = 400;

        // Поправка чтобы спрайт существа ровно располагался на спрайте клетки
        const X_CORRECTION: i32 = 30;
        const Y_CORRECTION: i32 = -50;

        let Self { left_margin, top_margin, width, height, ..} = *self;

        let full_rect = Rect::from_center(center, FULL_WIDTH, FULL_HEIGHT);
        let (reference_point, x_offset) =
            match side {
                Side::Attacker =>
                    (full_rect.top_left(), left_margin as i32 + X_CORRECTION),
                Side::Defender =>
                    (full_rect.top_right(), -((left_margin + width) as i32 + X_CORRECTION))
            };
        
        let top_left = reference_point.offset(x_offset, top_margin as i32 + Y_CORRECTION);
        Rect::new(top_left.x(), top_left.y(), width, height)
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}

type AnimationBlock = Box<[usize]>;


pub struct CreatureSpritesheet {
    pub colors: Box<[Color]>,
    pub sprites: Box<[CreatureSprite]>,
    pub blocks: [Option<AnimationBlock>; AnimationType::COUNT]
}

impl CreatureSpritesheet {
    const CREATURE_DEF_TYPE: u32 = 66;

    pub fn from_def_container(def_container: DefContainer) -> Self {
        let DefContainer { type_, mut colors, blocks2names, names2sprites } = def_container;

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
        let names2indexes = names.into_iter().enumerate().map(|(i, s)| (s, i)).collect::<HashMap<String, usize>>();
        let mut sprites = def_sprites.into_iter().map(CreatureSprite::from_def_sprite).collect::<Box<[CreatureSprite]>>();
        sprites.iter_mut().for_each(|sprite| sprite.apply_palette(&palette));
        
        // Блоки анимаций - последовательности индексов спрайтов 
        const NONE: Option<AnimationBlock> = None;
        let mut blocks = [NONE; AnimationType::COUNT];

        for (block_index, _) in AnimationType::iter().enumerate() {
            if let Some(block) = blocks2names.get(&(block_index as u32)) {
                let block = block.iter().map(|sprite_name| names2indexes[sprite_name]).collect::<AnimationBlock>();        
                blocks[block_index] = Some(block);
            }
        }

        Self { colors, sprites, blocks }
    }

    pub fn animation_block(&self, animation: AnimationType) -> &AnimationBlock {
        self.blocks[animation as usize].as_ref().unwrap()
    }
}