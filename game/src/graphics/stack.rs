use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use crate::battlestate::Side;
use crate::error::AnyWay;
use crate::grid::GridPos;
use crate::ResourceRegistry;

use crate::stack::Stack;

use super::animations::AnimationState;
use super::creature::AnimationType;
use super::Statics;

pub fn draw(
    stack: &Stack,
    animation_state: &AnimationState,
    canvas: &mut WindowCanvas,
    rr: &mut ResourceRegistry,
    tc: &TextureCreator<WindowContext>,
    is_selected: bool,
    statics: &Statics,
) -> AnyWay {
    let spritesheet = rr.get_creature_spritesheet(stack.creature);

    let animation_data = animation_state.get_state();

    let (animation_type, frame_index) = if stack.is_alive() || animation_state.is_animating() {
        (animation_data.type_, animation_data.frame_index)
    } else {
        let animation_type = AnimationType::Death;
        (
            animation_type,
            spritesheet.frames_count(animation_type).unwrap() - 1,
        )
    };

    let side = if animation_data.invert_side {
        stack.side.other()
    } else {
        stack.side
    };

    let offset_x = if stack.creature.is_wide() && !animation_data.invert_side {
        let offset = GridPos::CELL_WIDTH;
        match side {
            Side::Attacker => -offset,
            Side::Defender => offset,
        }
    } else {
        0
    };

    let draw_pos = animation_data.position.offset(offset_x, 0);

    spritesheet.draw(
        canvas,
        tc,
        draw_pos,
        side,
        is_selected,
        animation_type,
        frame_index,
    )?;

    if stack.is_alive() && !animation_state.is_animating() {
        draw_count(stack, canvas, tc, statics)?;
    }

    Ok(())
}

fn draw_count(
    logic: &Stack,
    canvas: &mut WindowCanvas,
    tc: &TextureCreator<WindowContext>,
    statics: &Statics,
) -> AnyWay {
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
