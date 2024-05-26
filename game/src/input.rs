use egui::RawInput;
use gamedata::creatures::Creature;
use logic::command;
use logic::command::{Cast, Command};
use logic::gamestate::GameState;
use logic::grid::{AttackDirection, GridPos};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Point;
use sdl2::EventPump;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{gridpos, Stage};

#[derive(Default)]
pub struct FrameInput {
    cursor_position: (i32, i32),
    btn_lmb: bool,
    btn_rmb: bool,
    key_d: bool,
    key_w: bool,
    quit: bool,
}

pub struct FrameData {
    pub current_hover: Option<GridPos>,
    pub potential_lmb_command: Option<Command>,
    pub command: Option<Command>,
}

#[derive(Clone, Copy, Debug, EnumIter)]
enum HexagonPart {
    Left,
    TopLeft,
    TopHalfLeft,
    TopHalfRight,
    TopRight,
    Right,
    BotRight,
    BotHalfRight,
    BotHalfLeft,
    BotLeft,
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
    stage: &mut Stage,
    cast: Option<Cast>,
) -> FrameData {
    if frame_input.quit {
        match stage {
            Stage::Main => std::process::exit(0),
            Stage::SpellBook => *stage = Stage::Main,
        }
    }

    match stage {
        Stage::Main => {
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
        Stage::SpellBook => FrameData {
            current_hover: None,
            potential_lmb_command: None,
            command: None,
        },
    }
}

pub fn to_raw_input(frame_input: &FrameInput) -> RawInput {
    let mut raw_input = RawInput::default();

    let cursor_pos = egui::pos2(
        frame_input.cursor_position.0 as f32,
        frame_input.cursor_position.1 as f32,
    );

    if frame_input.btn_lmb {
        raw_input.events.push(egui::Event::PointerButton {
            pos: cursor_pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        });
        raw_input.events.push(egui::Event::PointerButton {
            pos: cursor_pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        });
    }

    raw_input
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

impl HexagonPart {
    // Конец дуги соответствующей части
    // если идти по часовой стрелке
    fn arc_end(&self) -> f32 {
        use std::f32::consts::*;
        // [0; 2*PI]
        // Ноль - середина левой стороны
        // Идём по часовой стрелке
        match self {
            Self::Left => -(PI - FRAC_2_PI),
            Self::TopLeft => -(FRAC_PI_2 + FRAC_2_PI),
            Self::TopHalfLeft => -FRAC_PI_2,
            Self::TopHalfRight => -(FRAC_PI_2 - FRAC_2_PI),
            Self::TopRight => -FRAC_2_PI,
            Self::Right => FRAC_2_PI,
            Self::BotRight => FRAC_PI_2 - FRAC_2_PI,
            Self::BotHalfRight => FRAC_PI_2,
            Self::BotHalfLeft => FRAC_PI_2 + FRAC_2_PI,
            Self::BotLeft => PI - FRAC_2_PI,
        }
    }

    fn find_part_for_angle(angle: f32) -> Self {
        Self::iter()
            .find(|hex_part| angle < hex_part.arc_end())
            .unwrap_or(Self::Left)
    }

    fn to_attack_direction(self, attacking_creature: Creature) -> AttackDirection {
        match (self, attacking_creature.is_wide()) {
            (Self::Left, _) => AttackDirection::Left,
            (Self::Right, _) => AttackDirection::Right,
            (Self::TopHalfLeft, false) => AttackDirection::TopLeft,
            (Self::TopHalfLeft, true) => AttackDirection::Top,
            (Self::TopHalfRight, false) => AttackDirection::TopRight,
            (Self::TopHalfRight, true) => AttackDirection::Top,
            (Self::BotHalfLeft, false) => AttackDirection::BottomLeft,
            (Self::BotHalfLeft, true) => AttackDirection::Bottom,
            (Self::BotHalfRight, false) => AttackDirection::BottomRight,
            (Self::BotHalfRight, true) => AttackDirection::Bottom,
            (Self::BotLeft, _) => AttackDirection::BottomLeft,
            (Self::BotRight, _) => AttackDirection::BottomRight,
            (Self::TopLeft, _) => AttackDirection::TopLeft,
            (Self::TopRight, _) => AttackDirection::TopRight,
        }
    }
}
