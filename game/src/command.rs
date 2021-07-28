use super::creature_stack::CreatureTurnState as CTS;
use super::battlestate::{BattleState, CreatureStackHandle};
use super::gridpos::GridPos;

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum Command {
    Move { destination: GridPos },
    Wait,
    Defend,
    Shoot { target: CreatureStackHandle }
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq)]
enum CommandFieldless {
    Move,
    Wait,
    Defend,
    Shoot
}

impl Command {
    pub fn is_applicable(&self, state: &BattleState) -> bool {
        match *self {
            Self::Defend =>
                is_applicable_defend(),
            Self::Wait =>
                is_applicable_wait(state),
            Self::Move { destination } =>
                is_applicable_move(state, destination),
            Self::Shoot { target } => 
                is_applicable_shoot(state, target)
        }
    }
    pub fn apply(&self, state: &mut BattleState) {
        match *self {
            Self::Defend =>
                apply_defend(state),
            Self::Wait =>
                apply_wait(state),
            Self::Move { destination } =>
                apply_move(state, destination),
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
    fn fieldless(&self) -> CommandFieldless {
        match self {
            Self::Defend { .. }  => CommandFieldless::Defend,
            Self::Move   { .. }  => CommandFieldless::Move,
            Self::Shoot  { .. }  => CommandFieldless::Shoot,
            Self::Wait   { .. }  => CommandFieldless::Wait
        }
    }

    fn requires_current_stack_update(&self) -> bool {
        [
            CommandFieldless::Defend,
            CommandFieldless::Move,
            CommandFieldless::Shoot,
            CommandFieldless::Wait
        ].contains(&self.fieldless())
    }

    fn spends_turn(&self) -> bool {
        match self {
            Self::Wait => false,
            _ => true
        }
    }
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
    let side = state.current_stack.side;
    let current_stack = state.get_current_stack_mut();
    current_stack.set_head(side, destination);
}

fn is_applicable_shoot(state: &BattleState, target: CreatureStackHandle) -> bool {
    let has_ammo = state.get_current_stack().current_ammo > 0;
    has_ammo
}
fn apply_shoot(state: &mut BattleState, target: CreatureStackHandle) {
    let mut attack_stack = state.get_current_stack_mut();
    attack_stack.current_ammo -= 1;

    let mut defend_stack = state.get_stack_mut(target);
    defend_stack.count -= 1;
}
