use crate::command::Command;
use crate::event::Event;

use super::BattleState;

mod attack;
mod defend;
mod r#move;
mod shoot;
mod wait;

pub fn is_applicable(state: &BattleState, command: Command) -> bool {
    match command {
        Command::Defend(command) => defend::is_applicable(command, state),
        Command::Wait(command) => wait::is_applicable(command, state),
        Command::Move(command) => r#move::is_applicable(command, state),
        Command::Attack(command) => attack::is_applicable(command, state),
        Command::Shoot(command) => shoot::is_applicable(command, state),
    }
}

pub fn apply(state: &mut BattleState, command: Command) -> Vec<Event> {
    match command {
        Command::Defend(command) => defend::apply(command, state),
        Command::Wait(command) => wait::apply(command, state),
        Command::Move(command) => r#move::apply(command, state),
        Command::Attack(command) => attack::apply(command, state),
        Command::Shoot(command) => shoot::apply(command, state),
    }
}
