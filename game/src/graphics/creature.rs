use sdl2::pixels::{Color, Palette};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};

use gamedata::creatures;

use crate::resources::spritesheets::{ContainerType, Sprite, SpriteSheet, SpriteSheetType};
use logic::gamestate::Side;

impl ContainerType for creatures::Animation {
    const CONTAINER_TYPE: u32 = Self::CONTAINER_TYPE;
}

impl SpriteSheetType for creatures::Animation {
    fn block_index(&self) -> usize {
        *self as usize
    }

    fn container_index(&self) -> u32 {
        Self::container_index(*self)
    }
}

impl SpriteSheet<creatures::Animation> {
    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        tc: &TextureCreator<WindowContext>,
        draw_pos: Point,
        side: Side,
        is_selected: bool,
        animation_type: creatures::Animation,
        frame_index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sprite = self.get_sprite(animation_type, frame_index).unwrap();

        let draw_rect = draw_rect(sprite, draw_pos, side);

        let texture = if is_selected {
            with_selection(sprite, self).as_texture(tc)
        } else {
            sprite.surface.as_texture(tc)
        }?;

        match side {
            Side::Attacker => canvas.copy(&texture, None, draw_rect),
            Side::Defender => canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false),
        }?;

        Ok(())
    }
}

fn with_selection(
    sprite: &Sprite,
    spritesheet: &SpriteSheet<creatures::Animation>,
) -> Surface<'static> {
    let mut surface = sprite
        .surface
        .convert(&sprite.surface.pixel_format())
        .unwrap();

    let mut colors = spritesheet.colors().to_owned();
    colors[5] = Color::YELLOW;
    let palette = Palette::with_colors(&colors).unwrap();

    surface.set_palette(&palette).unwrap();

    surface
}

fn draw_rect(sprite: &Sprite, center: Point, side: Side) -> Rect {
    const FULL_WIDTH: u32 = 450;
    const FULL_HEIGHT: u32 = 400;

    // Поправка чтобы спрайт существа ровно располагался на спрайте клетки
    const X_CORRECTION: i32 = 30;
    const Y_CORRECTION: i32 = -50;

    let Sprite {
        left_margin,
        top_margin,
        width,
        height,
        ..
    } = *sprite;

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
