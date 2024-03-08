use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub const X_RANGE: RangeInclusive<i32> = 1..=15;
    pub const Y_RANGE: RangeInclusive<i32> = 1..=11;

    fn is_point_valid(x: i32, y: i32) -> bool {
        Self::X_RANGE.contains(&x) && Self::Y_RANGE.contains(&y)
    }

    pub fn new(x: i32, y: i32) -> Self {
        assert!(Self::is_point_valid(x, y));
        Self { x, y }
    }

    pub fn relative(&self, x_modif: i8, y_modif: i8) -> Self {
        Self::new(self.x + x_modif as i32, self.y + y_modif as i32)
    }

    pub fn try_new(x: i32, y: i32) -> Option<Self> {
        if Self::is_point_valid(x, y) {
            Some(Self::new(x, y))
        } else {
            None
        }
    }

    pub fn try_relative(&self, x_modif: i8, y_modif: i8) -> Option<Self> {
        Self::try_new(self.x + x_modif as i32, self.y + y_modif as i32)
    }

    pub fn is_even_row(&self) -> bool {
        self.y % 2 == 0
    }

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
                (x - 1, y + 1),
            ]
        } else {
            [
                (x - 1, y),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x, y + 1),
            ]
        }
        .iter()
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

use std::fmt;
impl fmt::Display for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl fmt::Debug for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
