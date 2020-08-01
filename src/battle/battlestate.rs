use std::time::{Instant, Duration};

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture, TextureValueError};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::pixels::{Color, Palette};

use crate::enumerations::{Battlefield, Creature, AnimationType, Misc};
use crate::resources::ResourceRegistry;
use crate::util::AnyError;

use super::GridPos;

pub struct BattleState<'a> {
    logic: Logic,
    graphics: Graphics<'a>
}

struct Logic {
    battlefield: Battlefield
}

// Постоянно используемые текстуры,
// которые нет смысла прокачивать сквозь кэш
struct Graphics<'a> {
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

    creature_animation: CreatureAnimation
}

struct CreatureAnimation {
    creature: Creature,
    type_: AnimationType,
    current_sprite_index: usize,
    change_at: Instant
}

const ANIMATION_PERIOD: Duration = Duration::from_millis(100);

impl CreatureAnimation {
    pub fn new(creature: Creature, type_: AnimationType) -> Self {
        CreatureAnimation{
            creature,
            type_,
            current_sprite_index: 0,
            change_at: Instant::now() + ANIMATION_PERIOD
        }
    }

    pub fn update(&mut self, now: Instant) {
        if now >= self.change_at {
            if self.current_sprite_index == 3 {
                self.current_sprite_index = 0;
            } else {
                self.current_sprite_index = self.current_sprite_index + 1;
            }
            let excess_time = now - self.change_at;
            self.change_at = now - excess_time + ANIMATION_PERIOD;
        }
    }

    pub fn get_texture<'a>(&self, rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>, AnyError> {
        let creature_def = rr.get_creature_container(self.creature);
        let block = creature_def.blocks2names.get_mut(&(self.type_ as u32)).unwrap();
        let sprite_name = &block[self.current_sprite_index];
        let sprite = creature_def.names2sprites.get_mut(sprite_name).unwrap();

        // Применяем прозрачность
        creature_def.colors[0] = Color::RGBA(0, 0, 0, 0);
        creature_def.colors[1] = Color::RGBA(0, 0, 0, 32);
        creature_def.colors[2] = Color::RGBA(0, 0, 0, 64);
        creature_def.colors[3] = Color::RGBA(0, 0, 0, 128);
        creature_def.colors[4] = Color::RGBA(0, 0, 0, 128);
        creature_def.colors[5] = Color::RGBA(0, 0, 0, 0);
        creature_def.colors[6] = Color::RGBA(0, 0, 0, 128);
        creature_def.colors[7] = Color::RGBA(0, 0, 0, 64);

        let palette = Palette::with_colors(&creature_def.colors)?;
        sprite.surface.set_palette(&palette)?;
        sprite.surface.set_color_key(true, Color::BLACK)?;
        let texture = sprite.surface.as_texture(tc)?;
        Ok(texture)
    }

    pub fn get_draw_rect(&self, rr: &mut ResourceRegistry) -> Rect {
        let creature_def = rr.get_creature_container(self.creature);
        let sprite = creature_def.get_sprite(self.type_ as u32, self.current_sprite_index);
        // dbg!((sprite.left_margin, sprite.top_margin, sprite.width, sprite.height, sprite.full_width, sprite.full_height));
        Rect::new(sprite.left_margin as i32, sprite.top_margin as i32, sprite.width, sprite.height)
    }
}

impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let graphics = Graphics {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?,

            creature_animation: CreatureAnimation::new(Creature::Champion, AnimationType::Standing)
        };
        let logic = Logic {
            battlefield,
        };
        let battlestate = BattleState { 
            logic,
            graphics
        };

        Ok(battlestate)
    }

    pub fn update(&mut self, now: Instant) {
        self.graphics.creature_animation.update(now);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        let Self { logic, graphics } = self;
        // Рисуем поле боя
        canvas.copy(&graphics.battlefield, None, Rect::new(0, 0, 800, 556))?;
        // Рисуем сетку
        self.draw_grid(canvas)?;

        // Рисуем существо
        let creature_texture = self.graphics.creature_animation.get_texture(rr, tc)?;
        let mut creature_draw_rect = self.graphics.creature_animation.get_draw_rect(rr);
        creature_draw_rect.set_x(creature_draw_rect.x() - 90);
        creature_draw_rect.set_y(creature_draw_rect.y() - 140);
        canvas.copy(&creature_texture, None, creature_draw_rect)?;

        Ok(())
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) -> Result<(), AnyError> {
        let cell_texture = &self.graphics.grid_cell;
        for x in GridPos::X_MIN ..= GridPos::X_MAX {
            for y in GridPos::Y_MIN ..= GridPos::Y_MAX {
                let draw_rect = GridPos::new(x, y).get_draw_rect();
                canvas.copy(&cell_texture, None, draw_rect)?;
            }
        }
        Ok(())
    }
}
