use std::collections::{HashSet, BinaryHeap};

use crate::gridpos::GridPos;

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

    pub fn get_reachable_cells(self, radius: u8) -> Vec<Self> {
        // Breadth-first search
        let mut reachable_cells = HashSet::with_capacity(Self::TOTAL_CELLS);
        reachable_cells.insert(self);

        let mut current_cells = vec![self];
        for _ in 0..radius {
            let successors = current_cells
                .iter()
                .flat_map(|cell| cell.get_successors())
                .collect::<HashSet<Self>>();
            
            let new_cells = successors
                .difference(&reachable_cells)
                .copied()
                .collect::<Vec<Self>>();
            
            reachable_cells.extend(new_cells.iter().copied());

            current_cells = new_cells;
        }
        reachable_cells.into_iter().collect()
    }

    pub fn shortest_path_heuristic(self, destination: GridPos) -> usize {
        let i = (destination.x - self.x).pow(2) + (destination.y - self.y).pow(2);
        (i as f32).sqrt().floor() as usize
    }

    pub fn get_shortest_path(self, destination: GridPos) -> Option<Vec<GridPos>> {
        // A-star search

        // dbg!((self, destination));

        #[derive(Eq, Debug)]
        struct ToSeeCell {
            previous_cell: GridPos,
            cost_to_here: usize,
            estimated_cost_to_goal: usize
        };
        use std::cmp::Ordering;
        impl Ord for ToSeeCell {
            fn cmp(&self, other: &Self) -> Ordering {
                match other.estimated_cost_to_goal.cmp(&self.estimated_cost_to_goal) {
                    Ordering::Equal => other.cost_to_here.cmp(&self.cost_to_here),
                    ord => ord
                }
            }
        }
        impl PartialOrd for ToSeeCell {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl PartialEq for ToSeeCell {
            fn eq(&self, other: &Self) -> bool {
                self.estimated_cost_to_goal == other.estimated_cost_to_goal &&
                self.cost_to_here == other.cost_to_here
            }
        }

        #[derive(Clone, Copy, Debug)]
        struct VisitedCell {
            came_from: GridPos,
            cost_to_here: usize
        };
        const X_MAX: usize = *GridPos::X_RANGE.end() as usize;
        const Y_MAX: usize = *GridPos::Y_RANGE.end() as usize;
        struct CellsArray([Option<VisitedCell>; X_MAX *Y_MAX]);
        impl CellsArray {
            fn new() -> Self {
                Self([None; X_MAX * Y_MAX])
            }
            fn cell_to_index(cell: GridPos) -> usize {
                ((cell.x - 1) * Y_MAX as i32 + (cell.y - 1)) as usize
            }
            fn get_cell(&self, cell: GridPos) -> Option<VisitedCell> {
                self.0[Self::cell_to_index(cell)]
            }
            fn put_cell(&mut self, cell: GridPos, visited_cell: VisitedCell) {
                self.0[Self::cell_to_index(cell)] = Some(visited_cell);
            }
        }

        let mut to_see = BinaryHeap::with_capacity(Self::TOTAL_CELLS);
        to_see.push(ToSeeCell {
            previous_cell: self,
            cost_to_here: 0,
            estimated_cost_to_goal: 0
        });

        let mut cells = CellsArray::new();
        cells.put_cell(self, VisitedCell { came_from: self, cost_to_here: usize::max_value() });

        while let Some(ToSeeCell { previous_cell, cost_to_here, .. }) = to_see.pop() {
            // dbg!((previous_cell, cost_to_here));
            let successors = {
                if previous_cell == destination {
                    let count = cells.0.iter().filter(|&&c| c.is_some()).count();
                    dbg!(count);
                    let mut path = Vec::with_capacity(cost_to_here);
                    path.push(previous_cell);
                    let mut previous_cell = previous_cell;
                    while previous_cell != self {
                        let VisitedCell { came_from, .. } = cells.get_cell(previous_cell).unwrap();
                        path.push(came_from);
                        previous_cell = came_from;
                    }
                    path.reverse();
                    dbg!(&path);
                    return Some(path)
                }
                let VisitedCell { cost_to_here: c, .. } = cells.get_cell(previous_cell).unwrap();
                // dbg!((cost_to_here, c));
                if cost_to_here > c {
                    continue;
                }
                previous_cell.get_successors()
            };
            for successor in successors {
                // dbg!(successor);
                let new_cost = cost_to_here + 1;
                let h; // heuristic(&successor)
                if let Some(visited_cell) = cells.get_cell(successor) {
                    if cost_to_here < visited_cell.cost_to_here {
                        h = successor.shortest_path_heuristic(destination);
                        // переписываем старый маршрут новымVisi
                        let visited_cell = VisitedCell { came_from: previous_cell, cost_to_here: new_cost };
                        // dbg!(visited_cell);
                        cells.put_cell(successor, visited_cell);
                    } else { 
                        // dbg!("continue");
                        continue;
                    }
                } else {
                    h = successor.shortest_path_heuristic(destination);
                    // переписываем старый маршрут новымVisi
                    let visited_cell = VisitedCell { came_from: previous_cell, cost_to_here: new_cost };
                    // dbg!(visited_cell);
                    cells.put_cell(successor, visited_cell);
                }

                let to_see_cell = ToSeeCell { previous_cell: successor, cost_to_here: new_cost, estimated_cost_to_goal: h };
                // dbg!(&to_see_cell);
                // сохраняем новую пограничную клетку
                to_see.push(to_see_cell);
            }
        }
        // dbg!("return none");
        None
    }
}
