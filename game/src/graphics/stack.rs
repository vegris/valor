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

    let (animation_type, animation_progress, invert_side, tween_pos) =
        if stack.is_alive() || stack.animation_queue.is_animating() {
            stack.animation_queue.get_animation()
        } else {
            (AnimationType::Death, 1.0, false, None)
        };

    let tail = pathfinding::tail_for(stack.creature, stack.side, stack.head)
        .unwrap()
        .center();

    let draw_pos = if let Some(draw_pos) = tween_pos {
        draw_pos
    } else {
        tail
    };

    let (side, draw_pos) = if invert_side {
        let head = stack.head.center();
        let x = tail.x - head.x;
        let y = tail.y - head.y;
        if stack.creature == gamedata::Creature::Griffin {
            dbg!((x, y));
        }
        (stack.side.other(), draw_pos.offset(-x, -y))
    } else {
        (stack.side, draw_pos)
    };

    spritesheet.draw(
        canvas,
        tc,
        draw_pos,
        side,
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
