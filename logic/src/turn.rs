use crate::gamestate::Side;

pub struct Turn {
    pub priority_side: Side,
    pub current_phase: Phase,
    pub phases: PhaseIterator,
}

impl Turn {
    const PRIORITY_SIDE: Side = Side::Attacker;

    pub fn new() -> Self {
        Self::build(Self::PRIORITY_SIDE)
    }

    pub fn next(&self) -> Self {
        println!("New turn!");
        Self::build(self.priority_side.other())
    }

    fn build(priority_side: Side) -> Self {
        let mut phases = Phase::iter();
        let current_phase = phases.next().unwrap();

        Self {
            priority_side,
            current_phase,
            phases,
        }
    }

    pub fn try_advance_phase(&mut self) -> bool {
        let next = self.phases.next();

        if let Some(phase) = next {
            self.current_phase = phase;
            println!("New turn phase: {:?}!", self.current_phase);
        }

        next.is_some()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Fresh,
    Wait,
}

type PhaseIterator = std::array::IntoIter<Phase, 2>;

impl Phase {
    fn iter() -> PhaseIterator {
        [Self::Fresh, Self::Wait].into_iter()
    }
}
