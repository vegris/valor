use std::ops::RangeInclusive;

extern crate sdl2;
use sdl2::rect::{Point, Rect};

pub struct GridPos {
    x: u8,
    y: u8
}

impl GridPos {
    pub const X_RANGE: RangeInclusive<u8> = 1..=15;
    pub const Y_RANGE: RangeInclusive<u8> = 1..=11;

    const CELL_WIDTH: u32 = 45;
    const CELL_HEIGHT: u32 = 52;
    const CELL_VERTICAL_SIDE_LENGTH: u32 = 32;

    const ODD_START_X: u32 = 81;
    const ODD_START_Y: u32 = 86;
    const EVEN_START_X: u32 = 59;
    const EVEN_START_Y: u32 = 128;

    fn is_x_valid(x: u8) -> bool {
        Self::X_RANGE.contains(&x)
    }
    fn is_y_valid(y: u8) -> bool {
        Self::Y_RANGE.contains(&y)
    }

    pub fn new(x: u8, y: u8) -> Self {
        assert!(Self::is_x_valid(x) && Self::is_y_valid(y));
        Self {x, y}
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
}