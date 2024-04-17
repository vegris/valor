use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use gamedata::heroes;

use crate::resources::spritesheets::{AnimationGroup, Sprite};
use logic::gamestate::Side;

impl AnimationGroup<heroes::Animation> {
    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        tc: &TextureCreator<WindowContext>,
        side: Side,
        animation_type: heroes::Animation,
        frame_index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sprite = self.get_sprite(animation_type, frame_index).unwrap();
        let draw_rect = draw_rect(sprite, side);
        let texture = sprite.surface.as_texture(tc)?;

        match side {
            Side::Attacker => canvas.copy(&texture, None, draw_rect),
            Side::Defender => canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false),
        }?;

        Ok(())
    }
}

fn draw_rect(sprite: &Sprite, side: Side) -> Rect {
    const FULL_WIDTH: u32 = 150;
    const FULL_HEIGHT: u32 = 175;

    const Y: i32 = 75;

    let x = match side {
        Side::Attacker => 50,
        Side::Defender => 785,
    };

    let center = Point::new(x, Y);

    let Sprite {
        left_margin,
        top_margin,
        width,
        height,
        ..
    } = *sprite;

    let full_rect = Rect::from_center(center, FULL_WIDTH, FULL_HEIGHT);
    let (reference_point, x_offset) = (full_rect.top_left(), left_margin as i32);

    let top_left = reference_point.offset(x_offset, top_margin as i32);
    Rect::new(top_left.x(), top_left.y(), width, height)
}
