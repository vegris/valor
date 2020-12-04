use super::creature_stack::CreatureTurnState as CTS;
use super::battlestate::{BattleState, Side, StrikeType};
use super::functions;
use super::gridpos::GridPos;
use super::action_queue::ActionQueue;

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
            },
            Self::Attack { position: pos, target: index } => {
                let maybe_path = cur_stack.position().get_shortest_path_to(pos);
                let target_creature = state.get_stack(side.other(), *index);

                state.current_side() == side &&
                target_creature.map_or(false, |target| {
                    target.get_adjacent_cells(side.other()).contains(pos)
                }) &&
                maybe_path.map_or(false, |path| path.len() <= cur_stack.speed() as usize)
            }
            _ => unimplemented!()
        }
    }
    fn apply(&self, side: Side, state: &mut BattleState) {
        match self {
            Self::Defend => {
                let cur_stack = state.get_current_stack_mut();
                println!("{} is defending!", cur_stack);
                cur_stack.defending = true;
            },
            Self::Wait => {
                let cur_stack = state.get_current_stack_mut();
                println!("{} is waiting!", cur_stack);
                cur_stack.turn_state = CTS::Waited;
            },
            Self::Move { destination: dest } => {
                let cur_stack = state.get_current_stack_mut();
                println!("{} moves from {} to {}", cur_stack, cur_stack.position(), dest);
                cur_stack.set_position(*dest);
            },
            Self::Attack { position: pos, target: index } => {
                {
                    let cur_stack = state.get_current_stack();
                    let target_stack = state.get_stack(side.other(), *index).unwrap();
                    let action_queue = ActionQueue::new();
                    let damage = functions::calculate_strike_damage(
                        state.get_army(side).hero(),
                        cur_stack,
                        state.get_army(side.other()).hero(),
                        target_stack,
                        StrikeType::Melee,
                        &action_queue
                    );
                    println!(
                        "{} moves from {} to {} and attacks {} for {} damage",
                        cur_stack, cur_stack.position(), pos, target_stack, damage
                    );
                    let target_stack = state.get_stack_mut(side.other(), *index).unwrap();
                    target_stack.receive_damage(damage);
                }
                {
                    let target_stack = state.get_stack(side.other(), *index).unwrap();
                    if target_stack.is_alive() {
                        let cur_stack = state.get_current_stack();
                        let action_queue = ActionQueue::new();
                        let damage = functions::calculate_strike_damage(
                            state.get_army(side.other()).hero(),
                            target_stack,
                            state.get_army(side).hero(),
                            cur_stack,
                            StrikeType::Melee,
                            &action_queue
                        );
                        println!("{} retaliates with {} damage", target_stack, damage);
                        let cur_stack = state.get_current_stack_mut();
                        cur_stack.receive_damage(damage);
                    }
                }
            }
            _ => unimplemented!()
        }

        if self.creature_spends_turn() {
            let cur_stack = state.get_current_stack_mut();
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
