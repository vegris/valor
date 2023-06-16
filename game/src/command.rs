use super::battlestate::CreatureStackHandle;
use crate::grid::{AttackDirection, GridPos};

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Move(Move),
    Wait(Wait),
    Defend(Defend),
    Attack(Attack),
    Shoot(Shoot),
}

#[derive(Clone, Copy, PartialEq)]
enum CommandFieldless {
    Move,
    Wait,
    Defend,
    Attack,
    Shoot,
}

impl Command {
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
        self.fieldless() != CommandFieldless::Wait
    }

    fn fieldless(&self) -> CommandFieldless {
        match self {
            Self::Defend { .. } => CommandFieldless::Defend,
            Self::Move { .. } => CommandFieldless::Move,
            Self::Shoot { .. } => CommandFieldless::Shoot,
            Self::Wait { .. } => CommandFieldless::Wait,
            Self::Attack { .. } => CommandFieldless::Attack,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub destination: GridPos,
}

#[derive(Clone, Copy, Debug)]
pub struct Wait;

#[derive(Clone, Copy, Debug)]
pub struct Defend;

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub attack_position: GridPos,
    pub attack_direction: AttackDirection,
}

#[derive(Clone, Copy, Debug)]
pub struct Shoot {
    pub target: CreatureStackHandle,
}
