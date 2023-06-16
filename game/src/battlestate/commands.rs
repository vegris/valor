use crate::command::Command;

use super::BattleState;

mod attack;
mod defend;
mod r#move;
mod shoot;
mod wait;

pub trait CommandT {
    fn is_applicable(self, state: &BattleState) -> bool;
    fn apply(self, state: &mut BattleState);
}

pub fn is_applicable(state: &BattleState, command: Command) -> bool {
    match command {
        Command::Defend(command) => command.is_applicable(state),
        Command::Wait(command) => command.is_applicable(state),
        Command::Move(command) => command.is_applicable(state),
        Command::Attack(command) => command.is_applicable(state),
        Command::Shoot(command) => command.is_applicable(state),
    }
}

pub fn apply(state: &mut BattleState, command: Command) {
    match command {
        Command::Defend(command) => command.apply(state),
        Command::Wait(command) => command.apply(state),
        Command::Move(command) => command.apply(state),
        Command::Attack(command) => command.apply(state),
        Command::Shoot(command) => command.apply(state),
    }
}
