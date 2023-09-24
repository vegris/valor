use gamedata::creatures::abilities::Ability;

use crate::battlestate::BattleState;
use crate::command::{Attack, Move};
use crate::pathfinding;

use crate::battlestate::damage::{deal_damage, AttackType};

use crate::event::{Attack as AttackEvent, Event, Strike};

use super::r#move;

const ATTACK_TYPE: AttackType = AttackType::Melee;

pub fn is_applicable(command: Attack, state: &BattleState) -> bool {
    let current_stack = state.get_current_stack();
    let current_side = current_stack.side;
    let is_wide = current_stack.creature.is_wide();

    let occupied_cells = pathfinding::get_occupied_cells_for(
        current_stack.creature,
        current_side,
        current_stack.head,
    )
    .unwrap();

    let potential_pos = pathfinding::unit_position_for_attack(
        command.attack_position,
        command.attack_direction,
        current_side,
        is_wide,
    );

    // успех в случае
    // 1. на позиции есть существо
    // 2. оно - враг
    // 3. атакующий может дойти до позиции атаки и поместиться там
    state
        .find_unit_for_cell(command.attack_position)
        .map(|handle| state.get_stack(handle))
        .filter(|stack| stack.side != current_side)
        .and(potential_pos)
        .filter(|creature_pos| {
            occupied_cells.contains(creature_pos)
                || r#move::is_applicable(
                    Move {
                        destination: *creature_pos,
                    },
                    state,
                )
        })
        .is_some()
}

pub fn apply(command: Attack, state: &mut BattleState) -> Vec<Event> {
    let mut events = vec![];
    let mut strikes = vec![];

    let current_stack = state.get_current_stack();
    let is_wide = current_stack.creature.is_wide();

    let initial_position = current_stack.head;

    let position = pathfinding::unit_position_for_attack(
        command.attack_position,
        command.attack_direction,
        current_stack.side,
        is_wide,
    )
    .unwrap();

    let move_events = r#move::apply(
        Move {
            destination: position,
        },
        state,
    );
    events.extend(move_events.clone());

    let defender_handle = state.find_unit_for_cell(command.attack_position).unwrap();

    let [attacker, defender] = state
        .stacks
        .get_many_mut([state.current_stack, defender_handle])
        .unwrap();

    deal_damage(&state.heroes, attacker, defender, ATTACK_TYPE);
    strikes.push(Strike {
        retaliation: false,
        lethal: !defender.is_alive(),
    });

    if defender.is_alive()
        && defender.retaliation_count.has_retaliation()
        && !attacker.creature.has_ability(Ability::NoRetaliation)
    {
        defender.retaliation_count.decrement();
        deal_damage(&state.heroes, defender, attacker, ATTACK_TYPE);
        strikes.push(Strike {
            retaliation: true,
            lethal: !attacker.is_alive(),
        });
    }

    if defender.is_alive()
        && attacker.is_alive()
        && attacker.creature.has_ability(Ability::DoubleStrike)
    {
        println!("Using double strike!");
        deal_damage(&state.heroes, attacker, defender, ATTACK_TYPE);
        strikes.push(Strike {
            retaliation: false,
            lethal: !attacker.is_alive(),
        });
    }

    let attack = Event::Attack(AttackEvent {
        attacker: state.current_stack,
        defender: defender_handle,
        strikes,
    });

    events.push(attack);

    if attacker.is_alive() && attacker.creature.has_ability(Ability::ReturnAfterStrike) {
        // FIXME: Use real movement logic instead
        let movement = TryInto::<[_; 1]>::try_into(move_events)
            .ok()
            .and_then(|[event]| {
                if let Event::Movement(movement) = event {
                    Some(movement)
                } else {
                    None
                }
            });

        if let Some(mut movement) = movement {
            attacker.head = initial_position;

            movement.path.reverse();
            events.push(Event::Movement(movement));
        }
    }

    events
}
