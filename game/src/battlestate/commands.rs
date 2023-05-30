use crate::{
    command::Command,
    grid::{AttackDirection, GridPos},
    pathfinding,
};

use super::{turns, BattleState, CreatureStackHandle};

pub fn is_applicable(state: &BattleState, command: Command) -> bool {
    match command {
        Command::Defend => is_applicable_defend(),
        Command::Wait => is_applicable_wait(state),
        Command::Move { destination } => is_applicable_move(state, destination),
        Command::Attack {
            attack_position,
            attack_direction,
        } => is_applicable_attack(state, attack_position, attack_direction),
        Command::Shoot { target } => is_applicable_shoot(state, target),
    }
}

pub fn apply(state: &mut BattleState, command: Command) {
    match command {
        Command::Defend => apply_defend(state),
        Command::Wait => apply_wait(state),
        Command::Move { destination } => apply_move(state, destination),
        Command::Attack {
            attack_position,
            attack_direction,
        } => apply_attack(state, attack_position, attack_direction),
        Command::Shoot { target } => apply_shoot(state, target),
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
    state
        .get_current_stack()
        .turn_state
        .map_or(false, |phase| phase == turns::Phase::Fresh)
}
fn apply_wait(state: &mut BattleState) {
    let current_stack = state.get_current_stack_mut();
    current_stack.turn_state = Some(turns::Phase::Wait);
}

fn is_applicable_move(state: &BattleState, destination: GridPos) -> bool {
    let current_stack = state.get_current_stack();

    let is_position_available = pathfinding::get_occupied_cells_for(
        current_stack.creature,
        current_stack.side,
        destination,
    )
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
    let _path = state
        .navigation_array
        .get_shortest_path(destination)
        .unwrap();

    let current_stack = state.get_current_stack_mut();

    current_stack.head = destination;
}

fn is_applicable_shoot(state: &BattleState, target: CreatureStackHandle) -> bool {
    let current_stack = state.get_current_stack();
    let target_stack = state.get_stack(target);

    let is_enemy = current_stack.side != target_stack.side;
    let is_alive = state.get_stack(target).is_alive();
    let can_shoot = current_stack.can_shoot(state);

    is_enemy && is_alive && can_shoot
}
fn apply_shoot(state: &mut BattleState, target: CreatureStackHandle) {
    let mut attack_stack = state.get_current_stack_mut();
    attack_stack.current_ammo -= 1;

    let mut defend_stack = state.get_stack_mut(target);
    defend_stack.count -= 1;
}

fn is_applicable_attack(
    state: &BattleState,
    attack_position: GridPos,
    attack_direction: AttackDirection,
) -> bool {
    let current_stack = state.get_current_stack();
    let current_side = current_stack.side;
    let is_wide = current_stack.creature.is_wide();

    let potential_pos = pathfinding::unit_position_for_attack(
        attack_position,
        attack_direction,
        current_side,
        is_wide,
    );

    // успех в случае
    // 1. на позиции есть существо
    // 2. оно - враг
    // 3. атакующий может дойти до позиции атаки и поместиться там
    state
        .find_unit_for_cell(attack_position)
        .map(|handle| state.get_stack(handle))
        .filter(|stack| stack.side != current_side)
        .and(potential_pos)
        .filter(|&creature_pos| is_applicable_move(state, creature_pos))
        .is_some()
}

fn apply_attack(
    state: &mut BattleState,
    attack_position: GridPos,
    attack_direction: AttackDirection,
) {
    let current_stack = state.get_current_stack();
    let is_wide = current_stack.creature.is_wide();

    let position = pathfinding::unit_position_for_attack(
        attack_position,
        attack_direction,
        current_stack.side,
        is_wide,
    )
    .unwrap();

    apply_move(state, position);

    let _current_stack = state.get_current_stack_mut();

    let defending_unit_handle = state.find_unit_for_cell(attack_position).unwrap();
    let mut defending_unit = state.get_stack_mut(defending_unit_handle);
    defending_unit.count -= 1;

    if defending_unit.is_alive() {
        let mut current_stack = state.get_current_stack_mut();
        current_stack.count -= 1;
    }
}
