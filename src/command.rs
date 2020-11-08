use super::creature_stack::CreatureTurnState as CTS;
use super::battlestate::{BattleState, Side};
use super::functions;
use super::gridpos::GridPos;

#[derive(Clone, Copy)]
pub struct Command {
    side: Side,
    type_: CommandType
}

impl Command {
    pub fn new(side: Side, type_: CommandType) -> Self {
        Self { side, type_ }
    }
    pub fn is_applicable(&self, state: &BattleState) -> bool {
        self.type_.is_applicable(self.side, state)
    }
    pub fn apply(&self, state: &mut BattleState) {
        self.type_.apply(self.side, state);
    }
}

#[derive(Clone, Copy)]
pub enum CommandType {
    Move { destination: GridPos },
    Wait,
    Defend,
    Attack { position: GridPos, target: u8 },
    Shoot { target: u8 }
}

impl CommandType {
    fn is_applicable(&self, side: Side, state: &BattleState) -> bool {
        match self {
            Self::Defend => {
                state.current_side() == side
            },
            Self::Wait => {
                let cur_stack = state.get_current_stack();
                let wait_states = [CTS::MoraledAndWaited, CTS::Waited];

                state.current_side() == side &&
                !wait_states.contains(&cur_stack.turn_state)
            },
            Self::Move { destination: dest } => {
                let cur_stack = state.get_current_stack();
                let maybe_path = cur_stack.position().get_shortest_path_to(dest);
                state.current_side() == side &&
                maybe_path.is_some() &&
                maybe_path.unwrap().len() <= cur_stack.speed().into()
            }
            _ => unimplemented!()
        }
    }
    fn apply(&self, side: Side, state: &mut BattleState) {
        match self {
            Self::Defend => {
                let cur_stack = state.get_current_stack_mut();
                cur_stack.defending = true;
                cur_stack.turn_state = CTS::NoTurn;
                println!("{} is defending!", cur_stack);
                state.update_current_stack();
            },
            Self::Wait => {
                let cur_stack = state.get_current_stack_mut();
                cur_stack.turn_state = CTS::Waited;
                println!("{} is waiting!", cur_stack);
                state.update_current_stack();
            },
            Self::Move { destination: dest } => {
                let mut cur_stack = state.get_current_stack_mut();
                println!("{} moves from {} to {}", cur_stack, cur_stack.position(), dest);
                cur_stack.set_position(*dest);
                cur_stack.turn_state = CTS::NoTurn;
                state.update_current_stack();
            }
            _ => unimplemented!()
        }
    }
}


// struct MoveCommand {
//     destination: GridPos
// }

// impl CommandType for MoveCommand {
//     fn is_applicable(&self, side: Side, state: &BattleState) -> bool {
//         side == state.current_side &&
//         state.current_stack.position().path_to(self.destination).is_some()
//     }
//     fn apply(&self, side: Side, state: &mut BattleState) {
//         state.current_stack.set_position(self.destination);
//         state.end_creature_turn();
//     }
// }
