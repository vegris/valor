use std::collections::VecDeque;
use std::convert::TryInto;

use crate::gridpos::{GridPos, AttackDirection};
use crate::battlestate::Side;

// Структуры для алгоритма Дейкстры
#[derive(Clone, Copy, Debug)]
struct VisitedCell {
    came_from: GridPos,
    cost_to_here: u32
}

const X_MAX: usize = *GridPos::X_RANGE.end() as usize;
const Y_MAX: usize = *GridPos::Y_RANGE.end() as usize;

pub struct NavigationArray([Option<VisitedCell>; X_MAX * Y_MAX]);

impl NavigationArray {
    pub fn empty() -> Self {
        Self([None; X_MAX * Y_MAX])
    }

    pub fn new(cell: GridPos) -> Self {
        let mut map = Self([None; X_MAX * Y_MAX]);
        map.put_cell(cell, cell, 0);

        let mut to_see: VecDeque<(GridPos, u32)> = VecDeque::new();
        to_see.push_back((cell, 0));

        while let Some((cell, cost_to_here)) = to_see.pop_front() {
            let new_cost = cost_to_here + 1;

            for successor in cell.get_successors() {
                // dbg!(successor);
                match map.get_cell(successor) {
                    // Уже видели вариант лучше
                    Some(seen_cell) if seen_cell.cost_to_here <= new_cost => {
                        continue;
                    },
                    // Этот вариант лучше
                    Some(_seen_cell) => {},
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

    pub fn get_shortest_path(&self, destination: GridPos) -> Vec<GridPos> {
        let mut path = vec![destination];

        let mut current_cell = destination;

        while let Some(visited_cell) = self.get_cell(current_cell) {
            // Клетка, из которой строился этот NavigationArray,
            // зациклена сама на себя
            // Значит, мы дошли до начала
            if current_cell == visited_cell.came_from {
                break;
            } else {
                current_cell = visited_cell.came_from;
                path.push(current_cell);
            }
        }

        path.pop();
        path.reverse();
        path
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
            cost_to_here: cost_to_here
        };
        self.0[Self::cell_to_index(cell)] = Some(visited_cell);
    }
}

impl GridPos {
    pub fn get_successors_positional(self) -> [Option<Self>; 6] {
        let Self { x, y } = self;

        // набор соседних клеток отличается в зависимости от чётности ряда
        if self.is_even_row() {
            [
                (x - 1, y), // начинаем слева и по часовой стрелке
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y),
                (x, y + 1),
                (x - 1, y + 1)
            ]
        } else {
            [
                (x - 1, y),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x, y + 1)
            ]
        }.iter()
         .map(|&(x, y)| Self::try_new(x, y))
         .collect::<Vec<Option<Self>>>()
         .try_into()
         .unwrap()
    }

    pub fn get_successors(self) -> Vec<Self> {
        self.get_successors_positional()
            .iter()
            .filter_map(|&x| x)
            .collect()
    }
}

// Нужна функция, определяющая положение существа при атаке
// Положение существа зависит от:
// 1. Атакуемого гекса
// 2. Направления атаки
// 3. Стороны существа
// 4. Ширины существа
// TODO: порефакторить это говно
pub fn unit_position_for_attack(
    attack_position: GridPos,
    attack_direction: AttackDirection,
    creature_side: Side,
    is_wide: bool
) -> Option<GridPos> {
    let successors = attack_position.get_successors_positional();

    let slim_creature_attack_directions = [
        AttackDirection::Left,
        AttackDirection::TopLeft,
        AttackDirection::TopRight,
        AttackDirection::Right,
        AttackDirection::BottomRight,
        AttackDirection::BottomLeft
    ];

    // Короткий путь до индекса с нужной клеткой
    // Всегда работает для обычных существ
    // Для широких есть дополнительные варианты
    let shortcut_gridpos_index =
            slim_creature_attack_directions
                .iter()
                .position(|&x| x == attack_direction);
    
    if is_wide {
        let gridpos_index =
            if let Some(gridpos_index) = shortcut_gridpos_index {
                gridpos_index    
            } else {
                match (attack_direction, creature_side) {
                    (AttackDirection::Top, Side::Attacker) => 1,
                    (AttackDirection::Top, Side::Defender) => 2,
                    (AttackDirection::Bottom, Side::Attacker) => 5,
                    (AttackDirection::Bottom, Side::Defender) => 4,
                    _ => unreachable!()
                }
            };
        let (x_modif, y_modif) =
            match (attack_direction, creature_side) {
                (AttackDirection::Left, Side::Attacker) => (-1, 0),
                (AttackDirection::TopLeft, Side::Attacker) => (-1, 0),
                (AttackDirection::TopRight, Side::Defender) => (1, 0),
                (AttackDirection::Right, Side::Defender) => (1, 0),
                (AttackDirection::BottomRight, Side::Defender) => (1, 0),
                (AttackDirection::BottomLeft, Side::Attacker) => (-1, 0),
                _ => (0, 0)
            };
        
        successors[gridpos_index].map(|gridpos| gridpos.relative(x_modif, y_modif))
    } else {
        successors[shortcut_gridpos_index.unwrap()]
    }
}
