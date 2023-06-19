use crate::Creature;

#[derive(Clone, Debug)]
pub enum RetaliationCount {
    Finite(i32),
    Infinite,
}

impl RetaliationCount {
    pub fn has_retaliation(&self) -> bool {
        !matches!(self, RetaliationCount::Finite(0))
    }

    pub fn decrement(&mut self) {
        if let RetaliationCount::Finite(n) = self {
            assert!(*n != 0);
            *n -= 1;
        }
    }
}

pub enum Ability {
    DoubleStrike,
    DoubleShot,
    Hatred { to: Box<[Creature]> },
    ExtraRetaliations { count: RetaliationCount },
}
