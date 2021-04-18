extern crate sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Point;

use crate::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use crate::gridpos::{GridPos, HexagonPart};
use crate::graphics::cursors::Cursor;
use crate::command::{Command, CommandType};

use super::{BattleState, Side};

#[derive(Default)]
pub struct FrameInput {
    cursor_position: (i32, i32),
    btn_lmb: bool,
    btn_rmb: bool,
    key_d: bool,
    key_w: bool,
    quit: bool
}

fn get_mouse_position(event_pump: &mut EventPump) -> (i32, i32) {
    let mouse_state = event_pump.mouse_state();
    (mouse_state.x(), mouse_state.y())
}

impl<'a> BattleState<'a> {
    pub fn gather_input(&self, event_pump: &mut EventPump) -> FrameInput {
        event_pump.pump_events();

        let mut frame_input = FrameInput {
            cursor_position: get_mouse_position(event_pump),
            ..Default::default()
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => frame_input.btn_lmb = true,
                        MouseButton::Right => frame_input.btn_rmb = true,
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::D => frame_input.key_d = true,
                        Keycode::W => frame_input.key_w = true,
                        Keycode::Escape => frame_input.quit = true,
                        _ => {}
                    }
                },
                Event::Quit {..} => frame_input.quit = true,
                _ => {}
            }
        }

        frame_input
    }

    pub fn process_input(&mut self, frame_input: FrameInput) {
        if frame_input.quit {
            std::process::exit(0);
        }

        let cursor_pos = frame_input.cursor_position;
        self.current_hover = GridPos::find_pointer_position(cursor_pos.into());
        self.hexagon_part = self.current_hover.map(|cell| cell.calculate_direction(cursor_pos.into()));

        if let Some(command) = self.construct_command(frame_input) {
            if command.is_applicable(self) {
                println!("Command applied!");
                command.apply(self);
            } else {
                println!("Command is not applicable!");
            }
        }
    }

    fn construct_command(&self, frame_input: FrameInput) -> Option<Command> {
        let command_type =
            if frame_input.key_d {
                Some(CommandType::Defend)
            } else if frame_input.key_w {
                Some(CommandType::Wait)
            } else if self.current_hover.is_some() && frame_input.btn_lmb {
                Some(CommandType::Move { destination: self.current_hover.unwrap() })
            } else {
                None
            };
        
        command_type.map(|type_| Command::new(self.current_side, type_))
    }
}
