use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use crate::{pathfinding, ResourceRegistry};

use crate::stack::Stack;

use crate::graphics::spritesheet::creature::AnimationType;

use super::Statics;

pub fn draw(
    stack: &Stack,
    canvas: &mut WindowCanvas,
    rr: &mut ResourceRegistry,
    tc: &TextureCreator<WindowContext>,
    is_selected: bool,
    statics: &Statics,
) -> Result<(), Box<dyn Error>> {
    let spritesheet = rr.get_creature_container(stack.creature);

    let (animation_type, animation_progress) =
        if stack.is_alive() || stack.animation_queue.is_animating() {
            stack.animation_queue.get_animation()
        } else {
            (AnimationType::Death, 1.0)
        };

    let draw_pos = pathfinding::tail_for(stack.creature, stack.side, stack.head)
        .unwrap()
        .center();

    spritesheet.draw(
        canvas,
        tc,
        draw_pos,
        stack.side,
        is_selected,
        animation_type,
        animation_progress,
    )?;

    if stack.is_alive() {
        draw_count(stack, canvas, tc, statics)?;
    }

    Ok(())
}

fn draw_count(
    logic: &Stack,
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
) -> Result<(), Box<dyn Error>> {
    let cell_center = logic.head.bounding_rect().center();
    let draw_center = cell_center.offset(0, 10);

    let background_texture = statics
        .textures
        .get(super::statics::StaticTexture::StackCountBackground);

    canvas.copy(
        background_texture,
        None,
        Rect::from_center(draw_center, 30, 11),
    )?;

    let font_surface = statics
        .font
        .render(&logic.count.to_string())
        .solid(Color::BLUE)?;
    let font_texture = font_surface.as_texture(tc)?;

    let mut font_rect = font_surface.rect();
    font_rect.center_on(draw_center);

    canvas.copy(&font_texture, None, font_rect)?;

    Ok(())
}
