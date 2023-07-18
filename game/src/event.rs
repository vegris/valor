use crate::{battlestate::StackHandle, grid::GridPos};

#[derive(Debug)]
pub struct Strike {
    pub retaliation: bool,
    pub lethal: bool,
}

#[derive(Debug)]
pub enum Event {
    Attack(Attack),
    Shot(Shot),
    Movement(Movement),
}

#[derive(Debug)]
pub struct Attack {
    pub attacker: StackHandle,
    pub defender: StackHandle,
    pub strikes: Vec<Strike>,
}

#[derive(Debug)]
pub struct Shot {
    pub attacker: StackHandle,
    pub target: StackHandle,
    pub lethal: bool,
}

#[derive(Debug)]
pub struct Movement {
    pub stack_handle: StackHandle,
    pub path: Vec<GridPos>,
}
