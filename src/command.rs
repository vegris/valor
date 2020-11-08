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

#[derive(Clone, Copy, PartialEq)]
enum CommandTypeFieldless {
    Move,
    Wait,
    Defend,
    Attack,
    Shoot
}

impl CommandType {
    fn is_applicable(&self, side: Side, state: &BattleState) -> bool {
        let cur_stack = state.get_current_stack();

        match self {
            Self::Defend => {
                state.current_side() == side
            },
            Self::Wait => {
                let wait_states = [CTS::MoraledAndWaited, CTS::Waited];

                state.current_side() == side &&
                !wait_states.contains(&cur_stack.turn_state)
            },
            Self::Move { destination: dest } => {
                let maybe_path = cur_stack.position().get_shortest_path_to(dest);

                state.current_side() == side &&
                maybe_path.is_some() &&
                maybe_path.unwrap().len() <= cur_stack.speed().into()
            }
            _ => unimplemented!()
        }
    }
    fn apply(&self, side: Side, state: &mut BattleState) {
        let cur_stack = state.get_current_stack_mut();

        match self {
            Self::Defend => {
                println!("{} is defending!", cur_stack);
                cur_stack.defending = true;
            },
            Self::Wait => {
                println!("{} is waiting!", cur_stack);
                cur_stack.turn_state = CTS::Waited;
            },
            Self::Move { destination: dest } => {
                println!("{} moves from {} to {}", cur_stack, cur_stack.position(), dest);
                cur_stack.set_position(*dest);
            }
            _ => unimplemented!()
        }

        if self.creature_spends_turn() {
            cur_stack.turn_state = CTS::NoTurn;
        }

        if self.requires_current_stack_update() {
            state.update_current_stack();
        }
    }

    // TODO: заменить эту каку макросом
    fn fieldless(&self) -> CommandTypeFieldless {
        match self {
            Self::Attack{ .. }  => CommandTypeFieldless::Attack,
            Self::Defend { .. } => CommandTypeFieldless::Defend,
            Self::Move { .. }   => CommandTypeFieldless::Move,
            Self::Shoot { .. }  => CommandTypeFieldless::Shoot,
            Self::Wait { .. }   => CommandTypeFieldless::Wait
        }
    }

    fn requires_current_stack_update(&self) -> bool {
        [
            CommandTypeFieldless::Attack,
            CommandTypeFieldless::Defend,
            CommandTypeFieldless::Move,
            CommandTypeFieldless::Shoot,
            CommandTypeFieldless::Wait
        ].contains(&self.fieldless())
    }

    fn creature_spends_turn(&self) -> bool {
        match self {
            Self::Wait => false,
            _ => true
        }
    }
}
