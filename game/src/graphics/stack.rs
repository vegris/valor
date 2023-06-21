use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::battlestate::Side;
use crate::graphics::spritesheet::creature::AnimationType;
use crate::graphics::spritesheet::Spritesheet;
use crate::{pathfinding, ResourceRegistry};

use super::spritesheet::creature;

pub fn draw(
    logic: &crate::stack::Stack,
    canvas: &mut WindowCanvas,
    rr: &mut ResourceRegistry,
    tc: &TextureCreator<WindowContext>,
    is_selected: bool,
    stack_count_bg: &Texture,
    font: &Font,
) -> Result<(), Box<dyn Error>> {
    let spritesheet = rr.get_creature_container(logic.creature);

    let animation_type = if logic.is_alive() {
        AnimationType::Standing
    } else {
        AnimationType::Death
    };

    let animation_progress = if animation_type == AnimationType::Death {
        1.0
    } else {
        0.0
    };

    let sprite = spritesheet
        .get_sprite(animation_type, animation_progress)
        .unwrap();
    let texture = if is_selected {
        creature::with_selection(sprite, spritesheet).as_texture(tc)
    } else {
        sprite.surface().as_texture(tc)
    }?;

    let draw_pos = pathfinding::tail_for(logic.creature, logic.side, logic.head)
        .unwrap()
        .center();

    let draw_rect = creature::draw_rect(sprite, draw_pos, logic.side);

    match logic.side {
        Side::Attacker => canvas.copy(&texture, None, draw_rect)?,
        Side::Defender => canvas.copy_ex(&texture, None, draw_rect, 0.0, None, true, false)?,
    };

    if logic.is_alive() {
        let cell_center = logic.head.bounding_rect().center();
        let draw_center = cell_center.offset(0, 10);
        canvas.copy(stack_count_bg, None, Rect::from_center(draw_center, 30, 11))?;

        let font_surface = font.render(&logic.count.to_string()).solid(Color::BLUE)?;
        let font_texture = font_surface.as_texture(tc)?;

        let mut font_rect = font_surface.rect();
        font_rect.center_on(draw_center);

        canvas.copy(&font_texture, None, font_rect)?;
    }

    Ok(())
}
