use std::collections::VecDeque;

use gamedata::Creature;

use crate::battlestate::{BattleState, Side};
use crate::grid::{AttackDirection, GridPos};

// Структуры для алгоритма Дейкстры
#[derive(Clone, Copy, Debug)]
struct VisitedCell {
    came_from: GridPos,
    cost_to_here: u32,
}

const X_MAX: usize = *GridPos::X_RANGE.end() as usize;
const Y_MAX: usize = *GridPos::Y_RANGE.end() as usize;

pub struct NavigationArray([Option<VisitedCell>; X_MAX * Y_MAX]);

impl NavigationArray {
    pub fn empty() -> Self {
        Self([None; X_MAX * Y_MAX])
    }

    pub fn new(cell: GridPos, state: &BattleState, is_flying: bool) -> Self {
        let mut map = Self([None; X_MAX * Y_MAX]);
        map.put_cell(cell, cell, 0);

        let mut to_see: VecDeque<(GridPos, u32)> = VecDeque::new();
        to_see.push_back((cell, 0));

        while let Some((cell, cost_to_here)) = to_see.pop_front() {
            let new_cost = cost_to_here + 1;

            let successors = cell.get_successors();

            let successors = if is_flying {
                successors
            } else {
                successors
                    .into_iter()
                    .filter(|x| state.find_unit_for_cell(*x).is_none())
                    .collect()
            };

            for successor in successors {
                match map.get_cell(successor) {
                    // Уже видели вариант лучше
                    Some(seen_cell) if seen_cell.cost_to_here <= new_cost => {
                        continue;
                    }
                    // Этот вариант лучше
                    Some(_seen_cell) => {}
                    // Встретили клетку впервые
                    None => {
                        to_see.push_back((successor, new_cost));
                    }
                }

                // Если пришли в клетку дешевле чем раньше (или впервые) -
                // записываем откуда пришли
                map.put_cell(successor, cell, new_cost);
            }
        }
        map
    }

    pub fn get_shortest_path(&self, destination: GridPos) -> Option<Vec<GridPos>> {
        let mut path = vec![destination];

        let mut current_cell = destination;

        while let Some(visited_cell) = self.get_cell(current_cell) {
            // Клетка, из которой строился этот NavigationArray,
            // зациклена сама на себя
            // Значит, мы дошли до начала
            if current_cell == visited_cell.came_from {
                path.reverse();
                return Some(path);
            } else {
                current_cell = visited_cell.came_from;
                path.push(current_cell);
            }
        }

        None
    }

    pub fn get_reachable_cells(&self, speed: u32) -> Vec<GridPos> {
        let mut reachable = vec![];

        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let cell = GridPos::new(x, y);
                if let Some(visited_cell) = self.get_cell(cell) {
                    if visited_cell.cost_to_here <= speed {
                        reachable.push(cell);
                    }
                }
            }
        }

        reachable
    }

    fn cell_to_index(cell: GridPos) -> usize {
        (cell.y - 1) as usize * X_MAX + (cell.x - 1) as usize
    }
    fn get_cell(&self, cell: GridPos) -> Option<VisitedCell> {
        self.0[Self::cell_to_index(cell)]
    }
    fn put_cell(&mut self, cell: GridPos, previous_cell: GridPos, cost_to_here: u32) {
        let visited_cell = VisitedCell {
            came_from: previous_cell,
            cost_to_here,
        };
        self.0[Self::cell_to_index(cell)] = Some(visited_cell);
    }
}

pub fn unit_position_for_attack(
    attack_position: GridPos,
    attack_direction: AttackDirection,
    creature_side: Side,
    is_wide: bool,
) -> Option<GridPos> {
    let position_index = match attack_direction {
        AttackDirection::Left => 0,
        AttackDirection::TopLeft => 1,
        AttackDirection::TopRight => 2,
        AttackDirection::Right => 3,
        AttackDirection::BottomRight => 4,
        AttackDirection::BottomLeft => 5,
        // Только для широких существ
        AttackDirection::Top => 1,
        AttackDirection::Bottom => 4,
    };
    let position = attack_position.get_successors_positional()[position_index];

    if is_wide {
        // Широкое существо для определённых направлений атаки
        // по умолчанию встаёт на клетку мимо
        // в таких случаях его нужно немного подвинуть
        let (directions, potential_adjustment) = match creature_side {
            Side::Attacker => {
                let directions = [
                    AttackDirection::Top,
                    AttackDirection::TopRight,
                    AttackDirection::Right,
                    AttackDirection::BottomRight,
                ];
                let adjustment = 1;
                (directions, adjustment)
            }
            Side::Defender => {
                let directions = [
                    AttackDirection::TopLeft,
                    AttackDirection::Left,
                    AttackDirection::BottomLeft,
                    AttackDirection::Bottom,
                ];
                let adjustment = -1;
                (directions, adjustment)
            }
        };

        let adjustment = if directions.contains(&attack_direction) {
            potential_adjustment
        } else {
            0
        };

        position.and_then(|x| x.try_relative(adjustment, 0))
    } else {
        position
    }
}

pub fn tail_for(creature: Creature, side: Side, head: GridPos) -> Option<GridPos> {
    if creature.is_wide() {
        match side {
            Side::Attacker => head.try_relative(-1, 0),
            Side::Defender => head.try_relative(1, 0),
        }
    } else {
        Some(head)
    }
}

pub fn head_from_tail(creature: Creature, side: Side, tail: GridPos) -> GridPos {
    if creature.is_wide() {
        match side {
            Side::Attacker => tail.relative(1, 0),
            Side::Defender => tail.relative(-1, 0),
        }
    } else {
        tail
    }
}

pub fn get_occupied_cells_for(
    creature: Creature,
    side: Side,
    head: GridPos,
) -> Option<Vec<GridPos>> {
    if creature.is_wide() {
        tail_for(creature, side, head).map(|tail| vec![head, tail])
    } else {
        Some(vec![head])
    }
}
