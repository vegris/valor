use std::ops::RangeInclusive;

extern crate sdl2;
use sdl2::rect::{Point, Rect};

extern crate pathfinding;
use pathfinding::directed::astar::astar;
use pathfinding::utils::absdiff;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    x: u16,
    y: u16
}

impl GridPos {
    pub const X_RANGE: RangeInclusive<u16> = 1..=15;
    pub const Y_RANGE: RangeInclusive<u16> = 1..=11;

    const CELL_WIDTH: u32 = 45;
    const CELL_HEIGHT: u32 = 52;
    const CELL_VERTICAL_SIDE_LENGTH: u32 = 32;

    const ODD_START_X: u32 = 81;
    const ODD_START_Y: u32 = 86;
    const EVEN_START_X: u32 = 59;
    const EVEN_START_Y: u32 = 128;

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

    fn is_even_row(&self) -> bool {
        self.y % 2 == 0
    }

    pub fn draw_pos(&self) -> Point {
        // Клетки нумеруются с первой, а алгоритм работает с нумерацией с нуля
        let (x, y) = (self.x as u32 - 1, self.y as u32 - 1);

        let base_x = x * (Self::CELL_WIDTH - 1); // Вычитаем единицу чтобы рисовать клетки "внахлёст"
        let base_y = y / 2 * (Self::CELL_HEIGHT + Self::CELL_VERTICAL_SIDE_LENGTH);

        let (x_pos, y_pos) = 
            if self.is_even_row() {
                (base_x + Self::EVEN_START_X, base_y + Self::EVEN_START_Y)
            } else {
                (base_x + Self::ODD_START_X, base_y + Self::ODD_START_Y)
            };

        Point::new(x_pos as i32, y_pos as i32)
    }

    pub fn draw_rect(&self) -> Rect {
        let draw_point = self.draw_pos();
        let (x, y) = (draw_point.x(), draw_point.y());
        Rect::new(x, y, Self::CELL_WIDTH, Self::CELL_HEIGHT)
    }

    pub fn contains_point(&self, point: (i32, i32)) -> bool {
        self.draw_rect().contains_point(point)
    }

    pub fn get_shortest_path_to(&self, destination: GridPos) -> Option<Vec<GridPos>> {
        astar(
            self,
            |p| p.get_successors().into_iter().map(|pos| (pos, 1)),
            |p| self.distance_to(*p), 
            |p| *p == destination
        ).map(|(vec, _cost)| vec)
    }

    fn distance_to(&self, p: Self) -> u16 {
        let (x, y) = (absdiff(self.x, p.x), absdiff(self.y, p.y));
        let sum_of_squares = x.pow(2) + y.pow(2);
        (sum_of_squares as f32).sqrt().ceil() as u16
    }

    fn get_successors(&self) -> Vec<Self> {
        let Self { x, y } = *self;

        // набор соседних клеток отличается в зависимости от чётности ряда
        if self.is_even_row() {
            vec![
                (x - 1, y), // начинаем слева и по часовой стрелке
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
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