use super::creature_stack::CreatureTurnState;
use super::battlestate::{BattleState, Side};
use super::functions;

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
    Move { destination: (u8, u8) },
    Wait,
    Defend,
    Attack { position: (u8, u8), target: u8 },
    Shoot { target: u8 }
}

impl CommandType {
    fn is_applicable(&self, side: Side, state: &BattleState) -> bool {
        match self {
            Self::Defend => {
                state.current_side() == side
            },
            _ => unimplemented!()
        }
    }
    fn apply(&self, side: Side, state: &mut BattleState) {
        match self {
            Self::Defend => {
                let cur_stack = state.get_current_stack_mut();
                cur_stack.defending = true;
                cur_stack.turn_state = CreatureTurnState::NoTurn;
                state.update_current_stack();
            },
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
