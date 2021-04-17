use std::collections::VecDeque;

use crate::gridpos::GridPos;

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
    pub fn get_successors(self) -> Vec<Self> {
        let Self { x, y } = self;

        // набор соседних клеток отличается в зависимости от чётности ряда
        if self.is_even_row() {
            vec![
                (x - 1, y), // начинаем слева и по часовой стрелке
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y),
                (x, y + 1),
                (x - 1, y + 1)
            ]
        } else {
            vec![
                (x - 1, y),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x, y + 1)
            ]
        }.into_iter()
         .filter_map(|(x, y)| Self::try_new(x, y))
         .collect()
    }
}

#[test]
fn name() {
    let start_cell = GridPos::new(3, 3);
    let navigation_array = NavigationArray::new(start_cell);
    dbg!(navigation_array.get_cell(GridPos::new(7, 7)));
    dbg!(navigation_array.get_shortest_path(GridPos::new(7, 7)));
}

#[test]
fn name_2() {
    let result = NavigationArray::cell_to_index(GridPos::new(3, 4));
    dbg!(result);
}
