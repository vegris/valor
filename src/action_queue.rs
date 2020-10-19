use std::convert::From;

use super::skills::Effect;

#[derive(PartialEq)]
pub enum Action {
    Move((u8, u8)),
    Proc(Effect)
}

pub struct ActionQueue(Vec<Action>);

impl ActionQueue {
    pub fn push_action(&mut self, action: Action) {
        self.0.push(action);
    }

    pub fn has_proc(&self, effect: Effect) -> bool {
        self.0.iter().find(|&e| matches!(*e, Action::Proc(eff) if eff == effect)).is_some()
    }

    pub fn cells_walked(&self) -> u8 {
        self.0.iter().filter(|e| matches!(e, Action::Move(_))).count() as u8
    }
}

impl From<Vec<Action>> for ActionQueue {
    fn from(v: Vec<Action>) -> Self {
        Self(v)
    }
}