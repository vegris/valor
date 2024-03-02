use gamedata::creatures::Ability;

use crate::gamestate::damage;
use crate::gamestate::GameState;
use crate::command::Shoot;
use crate::event::{Event, Shot};

const ATTACK_TYPE: damage::AttackType = damage::AttackType::Shoot;

pub fn is_applicable(command: Shoot, state: &GameState) -> bool {
    let current_stack = state.get_current_stack();
    let target_stack = state.get_stack(command.target);

    let is_enemy = current_stack.side != target_stack.side;
    let is_alive = state.get_stack(command.target).is_alive();
    let can_shoot = current_stack.can_shoot(state);

    is_enemy && is_alive && can_shoot
}
pub fn apply(command: Shoot, state: &mut GameState) -> Vec<Event> {
    let mut events = vec![];

    let [attacker, defender] = state
        .stacks
        .get_many_mut([state.current_stack, command.target])
        .unwrap();

    attacker.current_ammo -= 1;

    damage::deal_damage(&state.heroes, attacker, defender, ATTACK_TYPE);

    events.push(Event::Shot(Shot {
        attacker: state.current_stack,
        target: command.target,
        lethal: !defender.is_alive(),
    }));

    if !defender.is_alive() {
        return events;
    }

    if attacker.current_ammo > 0 && attacker.creature.has_ability(Ability::DoubleShot) {
        println!("Using double shot!");

        attacker.current_ammo -= 1;
        damage::deal_damage(&state.heroes, attacker, defender, ATTACK_TYPE);
        events.push(Event::Shot(Shot {
            attacker: state.current_stack,
            target: command.target,
            lethal: !defender.is_alive(),
        }));
    }

    events
}
