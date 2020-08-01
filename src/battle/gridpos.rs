extern crate sdl2;
use sdl2::rect::Rect;

pub struct GridPos {
    x: u8,
    y: u8
}

impl GridPos {
    pub const X_MIN: u8 = 1;
    pub const X_MAX: u8 = 15;
    pub const Y_MIN: u8 = 1;
    pub const Y_MAX: u8 = 11;

    const CELL_WIDTH: u32 = 45;
    const CELL_HEIGHT: u32 = 52;
    const CELL_VERTICAL_SIDE_LENGTH: u32 = 32;

    const ODD_START_X: u32 = 81;
    const ODD_START_Y: u32 = 86;
    const EVEN_START_X: u32 = 59;
    const EVEN_START_Y: u32 = 128;

    fn is_x_valid(x: u8) -> bool {
        x >= Self::X_MIN && x <= Self::X_MAX
    }
    fn is_y_valid(y: u8) -> bool {
        y >= Self::Y_MIN && y <= Self::Y_MAX
    }

    pub fn new(x: u8, y: u8) -> Self {
        assert!(Self::is_x_valid(x) && Self::is_y_valid(y));
        Self {x, y}
    }

    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn pos(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: u8) {
        assert!(Self::is_x_valid(x));
        self.x = x;
    }
    pub fn set_y(&mut self, y: u8) {
        assert!(Self::is_y_valid(y));
        self.y = y;
    }

    fn is_even_row(&self) -> bool {
        self.y % 2 == 0
    }

    pub fn get_draw_pos(&self) -> (i32, i32) {
        // Клетки нумеруются с первой, а алгоритм работает с нумерацией с нуля
        let (x, y) = (self.x as u32 - 1, self.y as u32 - 1);

        // TODO: Упростить цифродроч
        let (x_pos, y_pos) = 
            if self.is_even_row() {
                (
                    Self::EVEN_START_X + x * (Self::CELL_WIDTH - 1),
                    Self::EVEN_START_Y + ((y - 1) / 2) * (Self::CELL_HEIGHT + Self::CELL_VERTICAL_SIDE_LENGTH)
                )
            } else {
                (
                    Self::ODD_START_X + x * (Self::CELL_WIDTH - 1),
                    Self::ODD_START_Y + (y / 2) * (Self::CELL_HEIGHT + Self::CELL_VERTICAL_SIDE_LENGTH)
                )
            };
        (x_pos as i32, y_pos as i32)
    }

    pub fn get_draw_rect(&self) -> Rect {
        let (x_pos, y_pos) = self.get_draw_pos();
        Rect::new(x_pos, y_pos, Self::CELL_WIDTH, Self::CELL_HEIGHT)
    }
}