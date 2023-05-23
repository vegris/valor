use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

use crate::command::Command;
use crate::grid::GridPos;

#[derive(Default)]
pub struct FrameInput {
    pub cursor_position: (i32, i32),
    pub btn_lmb: bool,
    pub btn_rmb: bool,
    pub key_d: bool,
    pub key_w: bool,
    pub quit: bool,
}

pub struct FrameData {
    pub current_hover: Option<GridPos>,
    pub potential_lmb_command: Option<Command>,
}

pub fn gather_input(event_pump: &mut EventPump) -> FrameInput {
    event_pump.pump_events();

    let mut frame_input = FrameInput {
        cursor_position: get_mouse_position(event_pump),
        ..Default::default()
    };

    for event in event_pump.poll_iter() {
        match event {
            Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                MouseButton::Left => frame_input.btn_lmb = true,
                MouseButton::Right => frame_input.btn_rmb = true,
                _ => {}
            },
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::D => frame_input.key_d = true,
                Keycode::W => frame_input.key_w = true,
                Keycode::Escape => frame_input.quit = true,
                _ => {}
            },
            Event::Quit { .. } => frame_input.quit = true,
            _ => {}
        }
    }

    frame_input
}

fn get_mouse_position(event_pump: &mut EventPump) -> (i32, i32) {
    let mouse_state = event_pump.mouse_state();
    (mouse_state.x(), mouse_state.y())
}
