use super::creature_stack::CreatureTurnState as CTS;
use super::battlestate::{BattleState, CreatureStackHandle};
use super::gridpos::{GridPos, AttackDirection};
use crate::pathfinding::unit_position_for_attack;

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum Command {
    Move { destination: GridPos },
    Wait,
    Defend,
    Attack { attack_position: GridPos, attack_direction: AttackDirection},
    Shoot { target: CreatureStackHandle }
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq)]
pub enum CommandFieldless {
    Move,
    Wait,
    Defend,
    Attack,
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
            Self::Attack { attack_position, attack_direction } =>
                is_applicable_attack(state, attack_position, attack_direction),
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
            Self::Attack { attack_position, attack_direction } =>
                apply_attack(state, attack_position, attack_direction),
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
    pub fn fieldless(&self) -> CommandFieldless {
        match self {
            Self::Defend { .. } => CommandFieldless::Defend,
            Self::Move   { .. } => CommandFieldless::Move,
            Self::Shoot  { .. } => CommandFieldless::Shoot,
            Self::Wait   { .. } => CommandFieldless::Wait,
            Self::Attack { .. } => CommandFieldless::Attack
        }
    }

    fn requires_current_stack_update(&self) -> bool {
        [
            CommandFieldless::Defend,
            CommandFieldless::Move,
            CommandFieldless::Shoot,
            CommandFieldless::Wait,
            CommandFieldless::Attack
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
    let current_side = state.get_current_side();

    let is_position_available =
        current_stack
            .creature
            .get_occupied_cells_for(current_side, destination)
            .map(|cells| {
                cells
                    .into_iter()
                    .map(|cell| state.find_unit_for_cell(cell))
                    .all(|option| option.is_none())
            })
            .unwrap_or(false);

    is_position_available && state.reachable_cells.contains(&destination)
}
fn apply_move(state: &mut BattleState, destination: GridPos) {
    let current_stack = state.get_current_stack_mut();
    current_stack.head = destination;
}

fn is_applicable_shoot(state: &BattleState, target: CreatureStackHandle) -> bool {
    let current_stack = state.get_current_stack();
    let current_side = state.get_current_side();

    let is_enemy = current_side != target.side;
    let is_alive = state.get_stack(target).is_alive();
    let can_shoot = current_stack.can_shoot(current_side, state);

    is_enemy && is_alive && can_shoot
}
fn apply_shoot(state: &mut BattleState, target: CreatureStackHandle) {
    let mut attack_stack = state.get_current_stack_mut();
    attack_stack.current_ammo -= 1;

    let mut defend_stack = state.get_stack_mut(target);
    defend_stack.count -= 1;
}

fn is_applicable_attack(state: &BattleState, attack_position: GridPos, attack_direction: AttackDirection) -> bool {
    let current_stack = state.get_current_stack();
    let current_side = state.get_current_side();
    let is_wide = current_stack.creature.is_wide();

    let potential_pos = unit_position_for_attack(
        attack_position, attack_direction, current_side, is_wide
    );

    // успех в случае
    // 1. на позиции есть существо
    // 2. оно - враг
    // 3. атакующий может дойти до позиции атаки и поместиться там
    state
        .find_unit_for_cell(attack_position)
        .filter(|handle| handle.side != current_side)
        .and(potential_pos)
        .filter(|&creature_pos| is_applicable_move(state, creature_pos))
        .is_some()
}

fn apply_attack(state: &mut BattleState, attack_position: GridPos, attack_direction: AttackDirection) {
    let current_stack = state.get_current_stack();
    let current_side = state.get_current_side();
    let is_wide = current_stack.creature.is_wide();

    let position = unit_position_for_attack(
        attack_position, attack_direction, current_side, is_wide
    ).unwrap();

    apply_move(state, position);

    let defending_unit_handle = state.find_unit_for_cell(attack_position).unwrap();
    let mut defending_unit = state.get_stack_mut(defending_unit_handle);
    defending_unit.count -= 1;

    if defending_unit.is_alive() {
        let mut current_stack = state.get_current_stack_mut();
        current_stack.count -= 1;
    }
}
