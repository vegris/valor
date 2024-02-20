use gamedata::spells::Spell;

use crate::{battlestate::StackHandle, grid::GridPos};

#[derive(Debug, Clone)]
pub struct Strike {
    pub retaliation: bool,
    pub lethal: bool,
}

#[derive(Debug, Clone)]
pub enum Event {
    Attack(Attack),
    Shot(Shot),
    Movement(Movement),
    Cast(Cast),
}

#[derive(Debug, Clone)]
pub struct Attack {
    pub attacker: StackHandle,
    pub defender: StackHandle,
    pub strikes: Vec<Strike>,
}

#[derive(Debug, Clone)]
pub struct Shot {
    pub attacker: StackHandle,
    pub target: StackHandle,
    pub lethal: bool,
}

#[derive(Debug, Clone)]
pub struct Movement {
    pub stack_handle: StackHandle,
    pub path: Vec<GridPos>,
}

#[derive(Debug, Clone)]
pub struct Cast {
    pub spell: Spell,
    pub target: Option<GridPos>,
}
