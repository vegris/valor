use super::battlestate::CreatureStackHandle;
use crate::grid::{AttackDirection, GridPos};

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Move {
        destination: GridPos,
    },
    Wait,
    Defend,
    Attack {
        attack_position: GridPos,
        attack_direction: AttackDirection,
    },
    Shoot {
        target: CreatureStackHandle,
    },
}

#[derive(Clone, Copy, PartialEq)]
pub enum CommandFieldless {
    Move,
    Wait,
    Defend,
    Attack,
    Shoot,
}

impl Command {
    // TODO: заменить эту каку макросом
    pub fn fieldless(&self) -> CommandFieldless {
        match self {
            Self::Defend { .. } => CommandFieldless::Defend,
            Self::Move { .. } => CommandFieldless::Move,
            Self::Shoot { .. } => CommandFieldless::Shoot,
            Self::Wait { .. } => CommandFieldless::Wait,
            Self::Attack { .. } => CommandFieldless::Attack,
        }
    }

    pub fn requires_current_stack_update(&self) -> bool {
        [
            CommandFieldless::Defend,
            CommandFieldless::Move,
            CommandFieldless::Shoot,
            CommandFieldless::Wait,
            CommandFieldless::Attack,
        ]
        .contains(&self.fieldless())
    }

    pub fn spends_turn(&self) -> bool {
        !matches!(self, Self::Wait)
    }
}
