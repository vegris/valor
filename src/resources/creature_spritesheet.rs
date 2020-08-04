use std::mem::MaybeUninit;
use std::collections::HashMap;

extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::pixels::{Color, Palette};

use super::formats::{DefSprite, DefContainer};

use crate::battle::GridPos;

// Номера повторяют номера в реальном Def файле
#[derive(Clone, Copy)]
#[allow(unused, non_camel_case_types)]
pub enum Animation {
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


const BLOCKS_NUM: usize = 22;

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

    // pub fn get_draw_rect(&self) -> Rect;
    pub fn get_draw_rect_for_grid(&self, gridpos: &GridPos) -> Rect {
        let Self { left_margin, top_margin, width, height, .. } = *self;
        let (x_pos, y_pos) = gridpos.get_draw_pos();
        Rect::new(left_margin as i32 + x_pos - 173, top_margin as i32 + y_pos - 225, width, height)
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
}

type AnimationBlock = Box<[usize]>;


pub struct CreatureSpritesheet {
    colors: Box<[Color]>,
    sprites: Box<[CreatureSprite]>,
    blocks: [Option<AnimationBlock>; BLOCKS_NUM]
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
        let mut blocks: [MaybeUninit<Option<AnimationBlock>>; BLOCKS_NUM] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for elem in &mut blocks[..] {
            *elem = MaybeUninit::new(None);
        }
        let mut blocks = unsafe { std::mem::transmute::<_, [Option<AnimationBlock>; BLOCKS_NUM]>(blocks) };

        for animation_block_index in 0..BLOCKS_NUM {
            if let Some(block) = blocks2names.get(&(animation_block_index as u32)) {
                let block = block.iter().map(|sprite_name| names2indexes[sprite_name]).collect::<AnimationBlock>();        
                blocks[animation_block_index] = Some(block);
            }
        }

        Self { colors, sprites, blocks}
    }

    pub fn get_animation_block(&self, animation: Animation) -> Option<&AnimationBlock> {
        (&self.blocks[animation as usize]).as_ref()
    }
    pub fn get_sprite(&self, animation: Animation, sprite_num: usize) -> Option<&CreatureSprite> {
        self.get_animation_block(animation).map(|block| { 
            // Индекс спрайта в общем массиве спрайтов
            let sprite_index = block[sprite_num];
            &self.sprites[sprite_index] 
        })
    }
}