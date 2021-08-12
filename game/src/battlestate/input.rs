extern crate sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::gridpos::{GridPos, AttackDirection};
use crate::command::Command;

use super::BattleState;

#[derive(Default)]
pub struct FrameInput {
    cursor_position: (i32, i32),
    btn_lmb: bool,
    btn_rmb: bool,
    key_d: bool,
    key_w: bool,
    quit: bool
}

pub struct FrameData {
    pub current_hover: Option<GridPos>,
    pub attack_direction: Option<AttackDirection>,
    pub potential_lmb_command: Option<Command>
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

    pub fn process_input(&mut self, frame_input: FrameInput) -> FrameData {
        if frame_input.quit {
            std::process::exit(0);
        }

        let cursor_pos = frame_input.cursor_position;
        let current_hover = GridPos::find_pointer_position(cursor_pos.into());

        let current_stack = self.get_current_stack();
        let attack_direction = current_hover.map(|cell| cell.calculate_attack_direction(cursor_pos.into(), current_stack.creature));

        let potential_lmb_command = self.construct_potential_lmb_command(current_hover, attack_direction);

        if let Some(command) = self.construct_command(frame_input, potential_lmb_command) {
            dbg!(command);
            if command.is_applicable(self) {
                println!("Command applied!");
                command.apply(self);
            } else {
                println!("Command is not applicable!");
            }
        }

        FrameData { current_hover, attack_direction, potential_lmb_command }
    }

    fn construct_command(&self, frame_input: FrameInput, potential_lmb_command: Option<Command>) -> Option<Command> {
        if frame_input.key_d { return Some(Command::Defend) }
        if frame_input.key_w { return Some(Command::Wait) }
        if frame_input.btn_lmb { return potential_lmb_command }
        return None
    }

    fn construct_potential_lmb_command(
        &self,
        current_hover: Option<GridPos>,
        attack_direction: Option<AttackDirection>
    ) -> Option<Command> {
        let command =
            if let Some(gridpos) = current_hover {
                let current_stack = self.get_current_stack();

                if let Some(target) = self.find_unit_for_cell(gridpos) {
                    if current_stack.can_shoot() {
                        Some(Command::Shoot { target })
                    } else {
                        Some(Command::Attack { attack_position: gridpos, attack_direction: attack_direction.unwrap() })
                    }
                } else {
                    Some(Command::Move { destination: gridpos })
                }
            } else {
                None
            };
        command.filter(|c| c.is_applicable(self))
    }
}
