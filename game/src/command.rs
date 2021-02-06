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

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum CommandType {
    Move { destination: GridPos },
    Wait,
    Defend,
    Attack { position: GridPos, target: u8 },
    Shoot { target: u8 }
}

#[allow(unused)]
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
                state.current_side == side
            },
            Self::Wait => {
                state.current_side == side &&
                cur_stack.turn_state == CTS::HasTurn
            },
            Self::Move { destination: dest } => {
                let maybe_path = cur_stack.position.get_shortest_path_to(dest);
                let maybe_len = maybe_path.as_ref().map_or(100500, |x| x.len());
                println!("Shortest path from {} to {} is {:?}", cur_stack.position, dest, maybe_len);

                state.current_side == side &&
                maybe_path.is_some() &&
                maybe_path.unwrap().len() <= cur_stack.speed().into()
            },
            Self::Attack { position: pos, target: index } => {
                let maybe_path = cur_stack.position.get_shortest_path_to(pos);
                let target_creature = state.get_stack(side.other(), *index);

                state.current_side == side &&
                target_creature.map_or(false, |target| {
                    target.get_adjacent_cells(side.other()).contains(pos)
                }) &&
                maybe_path.map_or(false, |path| path.len() <= cur_stack.speed() as usize)
            },
            Self::Shoot { target: index } => {
                state.current_side == side &&
                state.get_stack(side.other(), *index).is_some() &&
                state.get_current_stack().current_ammo > 0
            }
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
                println!("{} moves from {} to {}", cur_stack, cur_stack.position, dest);
                cur_stack.position = *dest;
            },
            Self::Attack { position: _pos, target: index } => {
                let damage = make_strike(state, state.current_stack_id(), (side.other(), *index));
                let att_stack = state.get_current_stack();
                let def_stack = state.get_stack(side.other(), *index).unwrap();
                println!("{} attacks {} for {} damage", att_stack, def_stack, damage);
            },
            Self::Shoot { target: index } => {
                let damage = make_strike(state, state.current_stack_id(), (side.other(), *index));
                let att_stack = state.get_current_stack();
                let def_stack = state.get_stack(side.other(), *index).unwrap();
                println!("{} shoots {} for {} damage", att_stack, def_stack, damage);
            }
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

pub fn make_strike(state: &mut BattleState, attacker: (Side, u8), defender: (Side, u8)) -> u32 {
    let att_stack = state.get_stack(attacker.0, attacker.1).unwrap();
    let def_stack = state.get_stack(defender.0, defender.1).unwrap();

    let damage = functions::calculate_strike_damage(att_stack, def_stack);
    let def_stack_mut = state.get_stack_mut(defender.0, defender.1).unwrap();
    def_stack_mut.receive_damage(damage);

    damage
}
