use std::ops::RangeInclusive;

extern crate sdl2;
use sdl2::rect::{Point, Rect};

extern crate pathfinding;
use pathfinding::directed::bfs::bfs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: u16,
    pub y: u16
}

impl GridPos {
    pub const X_RANGE: RangeInclusive<u16> = 1..=15;
    pub const Y_RANGE: RangeInclusive<u16> = 1..=11;

    const CELL_WIDTH: u32 = 45;
    const CELL_HEIGHT: u32 = 52;
    const CELL_VERTICAL_SIDE_LENGTH: u32 = 32;

    const ODD_START_POINT: (i32, i32) = (105, 117);
    const EVEN_START_POINT: (i32, i32) = (83, 159);

    fn is_point_valid(x: u16, y: u16) -> bool {
        Self::X_RANGE.contains(&x) && Self::Y_RANGE.contains(&y)
    }

    pub fn new(x: u16, y: u16) -> Self {
        assert!(Self::is_point_valid(x, y));
        Self {x, y}
    }

    pub fn relative(&self, x_modif: i8, y_modif: i8) -> Self {
        Self::new((self.x as i32 + x_modif as i32) as u16, (self.y as i32 + y_modif as i32) as u16)
    }

    fn try_new(x: u16, y: u16) -> Option<Self> {
        if Self::is_point_valid(x, y) {
            Some(Self::new(x, y))
        } else {
            None
        }
    }

    pub fn try_relative(&self, x_modif: i8, y_modif: i8) -> Option<Self> {
        Self::try_new((self.x as i32 + x_modif as i32) as u16, (self.y as i32 + y_modif as i32) as u16)
    }

    pub fn is_even_row(&self) -> bool {
        self.y % 2 == 0
    }

    pub fn get_shortest_path_to(&self, destination: &GridPos) -> Option<Vec<GridPos>> {
        bfs(self, |p| p.get_successors(), |p| *p == *destination)
    }

    pub fn get_successors(&self) -> Vec<Self> {
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

    pub fn draw_center(&self) -> Point {
        let (x, y) = (self.x as u32 - 1, self.y as u32 - 1);

        // Вычитаем единицу чтобы рисовать клетки "внахлёст"
        let x_offset = x * (Self::CELL_WIDTH - 1);
        let y_offset = y / 2 * (Self::CELL_HEIGHT + Self::CELL_VERTICAL_SIDE_LENGTH);

        let start_point = 
            if self.is_even_row() {
                Self::EVEN_START_POINT
            } else {
                Self::ODD_START_POINT
            };
        
        Point::from(start_point).offset(x_offset as i32, y_offset as i32)
    }

    pub fn draw_rect(&self) -> Rect {
        Rect::from_center(self.draw_center(), Self::CELL_WIDTH, Self::CELL_HEIGHT)
    }

    pub fn contains_point(&self, point: (i32, i32)) -> bool {
        self.draw_rect().contains_point(point)
    }
}

use std::fmt;
impl fmt::Display for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}