use gamedata::creatures::Creature;
use sdl2::rect::Point;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

use crate::{gridpos, State};
use logic::command;
use logic::command::{Cast, Command};
use logic::gamestate::GameState;
use logic::grid::{AttackDirection, GridPos};

mod hexagon_part;

use hexagon_part::HexagonPart;

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
    pub command: Option<Command>,
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

pub fn process_input(
    state: &GameState,
    frame_input: &FrameInput,
    state2: &mut State,
    cast: Option<Cast>,
) -> FrameData {
    if frame_input.quit {
        match state2 {
            State::Main => std::process::exit(0),
            State::SpellBook => *state2 = State::Main,
        }
    }

    match state2 {
        State::Main => {
            let cursor_pos = frame_input.cursor_position;
            let current_hover = gridpos::find_pointer_position(cursor_pos.into());

            let current_stack = state.get_current_stack();
            let attack_direction = current_hover.map(|cell| {
                calculate_attack_direction(cell, cursor_pos.into(), current_stack.creature)
            });

            let potential_lmb_command =
                construct_potential_lmb_command(state, current_hover, attack_direction);

            let command = cast
                .map(Command::Cast)
                .or_else(|| construct_command(frame_input, potential_lmb_command));

            FrameData {
                current_hover,
                potential_lmb_command,
                command,
            }
        }
        State::SpellBook => FrameData {
            current_hover: None,
            potential_lmb_command: None,
            command: None,
        },
    }
}

fn get_mouse_position(event_pump: &mut EventPump) -> (i32, i32) {
    let mouse_state = event_pump.mouse_state();
    (mouse_state.x(), mouse_state.y())
}

fn construct_command(
    frame_input: &FrameInput,
    potential_lmb_command: Option<Command>,
) -> Option<Command> {
    if frame_input.key_d {
        return Some(Command::Defend);
    }
    if frame_input.key_w {
        return Some(Command::Wait);
    }
    if frame_input.btn_lmb {
        return potential_lmb_command;
    }
    None
}

fn construct_potential_lmb_command(
    state: &GameState,
    current_hover: Option<GridPos>,
    attack_direction: Option<AttackDirection>,
) -> Option<Command> {
    let current_stack = state.get_current_stack();

    current_hover
        .map(|grid| {
            if let Some(target) = state.find_unit_for_cell(grid) {
                if current_stack.can_shoot(state) {
                    Command::Shoot(command::Shoot { target })
                } else {
                    Command::Attack(command::Attack {
                        attack_position: grid,
                        attack_direction: attack_direction.unwrap(),
                    })
                }
            } else {
                Command::Move(command::Move { destination: grid })
            }
        })
        .filter(|c| state.is_command_applicable(*c))
}

pub fn calculate_attack_direction(
    pos: GridPos,
    point: Point,
    attacking_creature: Creature,
) -> AttackDirection {
    let grid_center = gridpos::center(pos);
    let point = point - grid_center;
    let x = point.x() as f32;
    let y = point.y() as f32;
    let angle = f32::atan2(y, x);
    HexagonPart::find_part_for_angle(angle).to_attack_direction(attacking_creature)
}
