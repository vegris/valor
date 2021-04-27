use super::creature_stack::{CreatureStack, CreatureTurnState as CTS};
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
        self.type_.apply(state);
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
        // Пока нет команд, которые можно исполнять в чужой ход
        if state.current_side != side { return false };

        match *self {
            Self::Defend =>
                is_applicable_defend(),
            Self::Wait =>
                is_applicable_wait(state),
            Self::Move { destination } =>
                is_applicable_move(state, destination),
            Self::Attack { position, target } =>
                is_applicable_attack(state, side, position, target),
            Self::Shoot { target } => 
                is_applicable_shoot(state, side, target)
        }
    }
    fn apply(&self, state: &mut BattleState) {
        match *self {
            Self::Defend =>
                apply_defend(state),
            Self::Wait =>
                apply_wait(state),
            Self::Move { destination } =>
                apply_move(state, destination),
            Self::Attack { target, .. } =>
                apply_attack(state, target),
            Self::Shoot { target } =>
                apply_shoot(state, target)
        }

        if self.spends_turn() {
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
            Self::Attack { .. }  => CommandTypeFieldless::Attack,
            Self::Defend { .. }  => CommandTypeFieldless::Defend,
            Self::Move   { .. }  => CommandTypeFieldless::Move,
            Self::Shoot  { .. }  => CommandTypeFieldless::Shoot,
            Self::Wait   { .. }  => CommandTypeFieldless::Wait
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

    fn spends_turn(&self) -> bool {
        match self {
            Self::Wait => false,
            _ => true
        }
    }
}

fn make_strike(state: &mut BattleState, attacker: (Side, u8), defender: (Side, u8)) -> u32 {
    let att_stack = state.get_stack(attacker.0, attacker.1).unwrap();
    let def_stack = state.get_stack(defender.0, defender.1).unwrap();

    let damage = functions::calculate_strike_damage(att_stack, def_stack);
    let def_stack_mut = state.get_stack_mut(defender.0, defender.1).unwrap();
    def_stack_mut.receive_damage(damage);

    damage
}

fn is_applicable_defend() -> bool {
    true
}
fn apply_defend(state: &mut BattleState) {
    let current_stack = state.get_current_stack_mut();
    current_stack.defending = true;
}

fn is_applicable_wait(state: &BattleState) -> bool {
    state.get_current_stack().turn_state == CTS::HasTurn
}
fn apply_wait(state: &mut BattleState) {
    let current_stack = state.get_current_stack_mut();
    current_stack.turn_state = CTS::Waited;
}

fn is_applicable_move(state: &BattleState, destination: GridPos) -> bool {
    let current_stack = state.get_current_stack();
    let path = state.navigation_array.get_shortest_path(destination);
    
    path.len() <= current_stack.speed().into()
}
fn apply_move(state: &mut BattleState, destination: GridPos) {
    let current_side = state.current_side;
    let current_stack = state.get_current_stack_mut();
    current_stack.set_head(current_side, destination);
}

fn is_applicable_attack(state: &BattleState, side: Side, position: GridPos, target: u8) -> bool {
    let path = state.navigation_array.get_shortest_path(position);
    let current_stack = state.get_current_stack();
    let target_creature = state.get_stack(side.other(), target);

    let is_position_near_target_func = |target: &CreatureStack| {
        target.get_adjacent_cells(side.other()).contains(&position)
    };
    let is_position_near_target = target_creature.map_or(false, is_position_near_target_func);

    let has_enough_speed = path.len() <= current_stack.speed().into();

    is_position_near_target && has_enough_speed
}
fn apply_attack(state: &mut BattleState, target: u8) {
    let current_side = state.current_side;

    let attack_stack_id = state.current_stack_id();
    let defend_stack_id = (current_side.other(), target);

    let damage = make_strike(state, attack_stack_id, defend_stack_id);

    let defend_stack = state.get_stack_mut(current_side.other(), target).unwrap();
    defend_stack.receive_damage(damage);
}

fn is_applicable_shoot(state: &BattleState, side: Side, target: u8) -> bool {
    let has_target = state.get_stack(side.other(), target).is_some();
    let has_ammo = state.get_current_stack().current_ammo > 0;

    has_target && has_ammo
}
fn apply_shoot(state: &mut BattleState, target: u8) {
    let current_side = state.current_side;

    let attack_stack_id = state.current_stack_id();
    let defend_stack_id = (current_side.other(), target);

    let damage = make_strike(state, attack_stack_id, defend_stack_id);

    let defend_stack = state.get_stack_mut(current_side.other(), target).unwrap();
    defend_stack.receive_damage(damage);
}
