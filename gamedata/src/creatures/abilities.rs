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

    ReturnAfterStrike,

    NoRetaliation,
    ExtraRetaliations { count: RetaliationCount },

    Hatred { to: Box<[Creature]> },

    IgnoreDefence { percent: f32 },
}

impl Creature {
    pub fn retaliation_count(self) -> RetaliationCount {
        self.abilities()
            .into_iter()
            .find_map(|ability| {
                if let Ability::ExtraRetaliations { count } = ability {
                    let count = match count {
                        RetaliationCount::Finite(n) => RetaliationCount::Finite(n + 1),
                        RetaliationCount::Infinite => RetaliationCount::Infinite,
                    };
                    Some(count)
                } else {
                    None
                }
            })
            .unwrap_or(RetaliationCount::Finite(1))
    }

    pub fn ignore_defence(self) -> Option<f32> {
        self.abilities().into_iter().find_map(|ability| {
            if let Ability::IgnoreDefence { percent } = ability {
                Some(percent)
            } else {
                None
            }
        })
    }
}
