use gamedata::spells::Spell;
use strum_macros::EnumDiscriminants;

use super::battlestate::StackHandle;
use crate::grid::{AttackDirection, GridPos};

#[derive(Clone, Copy, Debug, EnumDiscriminants)]
#[strum_discriminants(vis())]
pub enum Command {
    Move(Move),
    Wait,
    Defend,
    Attack(Attack),
    Shoot(Shoot),
    Cast(Cast),
}

impl Command {
    pub fn requires_current_stack_update(&self) -> bool {
        [
            CommandDiscriminants::Defend,
            CommandDiscriminants::Move,
            CommandDiscriminants::Shoot,
            CommandDiscriminants::Wait,
            CommandDiscriminants::Attack,
        ]
        .contains(&self.into())
    }

    pub fn spends_turn(&self) -> bool {
        Into::<CommandDiscriminants>::into(self) != CommandDiscriminants::Wait
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub destination: GridPos,
}

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub attack_position: GridPos,
    pub attack_direction: AttackDirection,
}

#[derive(Clone, Copy, Debug)]
pub struct Shoot {
    pub target: StackHandle,
}

#[derive(Clone, Copy, Debug)]
pub struct Cast {
    pub spell: Spell,
    pub target: Option<GridPos>,
}
