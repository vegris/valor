use std::ops::RangeInclusive;

extern crate pathfinding;
use pathfinding::directed::bfs::bfs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    x: u16,
    y: u16
}

impl GridPos {
    pub const X_RANGE: RangeInclusive<u16> = 1..=15;
    pub const Y_RANGE: RangeInclusive<u16> = 1..=11;

    fn is_point_valid(x: u16, y: u16) -> bool {
        Self::X_RANGE.contains(&x) && Self::Y_RANGE.contains(&y)
    }

    pub fn new(x: u16, y: u16) -> Self {
        assert!(Self::is_point_valid(x, y));
        Self {x, y}
    }

    fn try_new(x: u16, y: u16) -> Option<Self> {
        if Self::is_point_valid(x, y) {
            Some(Self::new(x, y))
        } else {
            None
        }
    }

    pub fn is_even_row(&self) -> bool {
        self.y % 2 == 0
    }

    pub fn get_shortest_path_to(&self, destination: GridPos) -> Option<Vec<GridPos>> {
        bfs(self, |p| p.get_successors(), |p| *p == destination)
    }

    fn get_successors(&self) -> Vec<Self> {
        let Self { x, y } = *self;

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

    pub fn x(&self) -> u16 {
        self.x
    }
}