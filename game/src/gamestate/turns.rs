use super::{GameState, Side, StackHandle};
use crate::stack::Stack;

pub fn find_active_stack(state: &GameState) -> Option<StackHandle> {
    let mut handles: Box<[StackHandle]> = state.stacks.keys().copied().collect();
    // Преимущество при равенстве скоростей у того кто ходил вторым на прошлом ходу
    handles
        .sort_unstable_by_key(|&handle| state.get_stack(handle).side == state.turn.priority_side);

    handles
        .iter()
        .map(|&handle| (handle, state.get_stack(handle)))
        .filter(|(_handle, stack)| stack.is_alive())
        .filter(|(_handle, stack)| {
            stack
                .turn_state
                .map_or(false, |phase| phase == state.turn.current_phase)
        })
        .fold(None, |acc, current| {
            // Без max_first тяжко
            fn key((_, stack): (StackHandle, &Stack)) -> i32 {
                stack.speed()
            }
            match acc {
                None => Some(current),
                Some(acc) if key(current) > key(acc) => Some(current),
                _ => acc,
            }
        })
        .map(|(handle, _stack)| handle)
}

pub struct Turn {
    priority_side: Side,
    current_phase: Phase,
    phases: PhaseIterator,
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
