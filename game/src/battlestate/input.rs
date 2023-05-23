use crate::{
    command::Command,
    grid::{AttackDirection, GridPos},
    input::{FrameData, FrameInput},
};

use super::BattleState;

impl BattleState {
    pub fn process_input(&mut self, frame_input: FrameInput) -> FrameData {
        if frame_input.quit {
            std::process::exit(0);
        }

        let cursor_pos = frame_input.cursor_position;
        let current_hover = GridPos::find_pointer_position(cursor_pos.into());

        let current_stack = self.get_current_stack();
        let attack_direction = current_hover
            .map(|cell| cell.calculate_attack_direction(cursor_pos.into(), current_stack.creature));

        let mut potential_lmb_command =
            self.construct_potential_lmb_command(current_hover, attack_direction);

        if let Some(command) = self.construct_command(frame_input, potential_lmb_command) {
            if command.is_applicable(self) {
                println!("Command applied!");
                command.apply(self);
                if let Some(winner) = self.find_winner() {
                    println!("{:?} wins!", winner);
                    std::process::exit(0);
                }

                // Если команда поменяла игровое состояние,
                // то есть шанс, что в potentail_lmb_command
                // стала содержаться устаревшая инфа
                // Лучше сбросить от греха
                potential_lmb_command = None;
            } else {
                println!("Command is not applicable!");
            }
        }

        FrameData {
            current_hover,
            potential_lmb_command,
        }
    }

    fn construct_command(
        &self,
        frame_input: FrameInput,
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
        &self,
        current_hover: Option<GridPos>,
        attack_direction: Option<AttackDirection>,
    ) -> Option<Command> {
        let command = if let Some(grid) = current_hover {
            let current_stack = self.get_current_stack();

            if let Some(target) = self.find_unit_for_cell(grid) {
                if current_stack.can_shoot(self) {
                    Some(Command::Shoot { target })
                } else {
                    Some(Command::Attack {
                        attack_position: grid,
                        attack_direction: attack_direction.unwrap(),
                    })
                }
            } else {
                Some(Command::Move { destination: grid })
            }
        } else {
            None
        };
        command.filter(|c| c.is_applicable(self))
    }
}
