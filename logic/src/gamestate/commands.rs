use crate::command::Command;
use crate::event::Event;

use super::GameState;

mod attack;
mod cast;
mod defend;
mod r#move;
mod shoot;
mod wait;

pub fn is_applicable(state: &GameState, command: Command) -> bool {
    match command {
        Command::Defend => defend::is_applicable(state),
        Command::Wait => wait::is_applicable(state),
        Command::Move(command) => r#move::is_applicable(command, state),
        Command::Attack(command) => attack::is_applicable(command, state),
        Command::Shoot(command) => shoot::is_applicable(command, state),
        Command::Cast(command) => cast::is_applicable(command, state),
    }
}

pub fn apply(state: &mut GameState, command: Command) -> Vec<Event> {
    match command {
        Command::Defend => defend::apply(state),
        Command::Wait => wait::apply(state),
        Command::Move(command) => r#move::apply(command, state),
        Command::Attack(command) => attack::apply(command, state),
        Command::Shoot(command) => shoot::apply(command, state),
        Command::Cast(command) => cast::apply(command, state),
    }
}
