use crate::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use crate::pathfinding::NavigationArray;

use super::{BattleState, Side, CreatureStackHandle};


pub type PhaseIterator = std::vec::IntoIter<CTS>;
pub fn new_phase_iter() -> PhaseIterator {
    vec![CTS::HasTurn, CTS::Waited].into_iter()
}

impl<'a> BattleState<'a> {
    pub fn update_current_stack(&mut self) {
        if let Some(handle) = self.find_current_creature() {
            self.current_stack = handle;

            let mut stack = self.get_current_stack_mut();
            stack.defending = false;
            println!("Current stack is {}, {:?}", stack, handle.side);

            let stack_head = stack.head;
            let is_flying = stack.creature.is_flying();
            let stack_speed = stack.speed().into();

            let navigation_array = NavigationArray::new(stack_head, self, is_flying);
            let reachable_cells = navigation_array.get_reachable_cells(stack_speed);
            self.navigation_array = navigation_array;
            self.reachable_cells = reachable_cells;
        } else {
            self.advance_phase();
            self.update_current_stack();
        }
    }

    pub fn advance_phase(&mut self) {
        if let Some(phase) = self.phase_iter.next() {
            self.current_phase = phase;
            println!("New turn phase: {:?}!", self.current_phase);
        } else {
            self.new_turn();
            self.advance_phase();
        }
    }

    pub fn new_turn(&mut self) {
        self.phase_iter = new_phase_iter();
        self.sides
            .iter_mut()
            .flatten()
            .for_each(|creature| creature.turn_state = CTS::HasTurn);
        self.last_turn_side = self.last_turn_side.other();
        println!("New turn!");
    }

    fn find_current_creature(&self) -> Option<CreatureStackHandle> {
        // Преимущество при равенстве скоростей у того кто ходил вторым на прошлом ходу
        match self.last_turn_side {
            Side::Attacker => vec![Side::Defender, Side::Attacker],
            Side::Defender => vec![Side::Attacker, Side::Defender]
        }
        .into_iter()
        .flat_map(|side| Iterator::zip(
            std::iter::repeat(side),
            self.sides[side as usize].iter().enumerate()
        ))
        .map(|(side, (index, stack))| (side, index, stack)) // чтоб не утонуть в скобках
        .filter(|(_side, _index, stack)| stack.is_alive())
        .filter(|(_side, _index, stack)| stack.turn_state == self.current_phase)
        .fold(None, |acc, current| {
            // Без max_first тяжко
            fn key((_, _, stack): (Side, usize, &CreatureStack)) -> u8 {
                stack.speed()
            }
            match acc {
                None => Some(current),
                Some(acc) if key(current) > key(acc) => Some(current),
                _ => acc
            }
        })
        .map(|(side, index, _stack)| CreatureStackHandle { side, index: index as u8 })
    }
}
