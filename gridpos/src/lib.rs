use std::ops::RangeInclusive;

extern crate sdl2;
use sdl2::rect::{Point, Rect};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use creature::Creature;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackDirection {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft
}

impl AttackDirection {
    fn from_hexagon_part(hexagon_part: HexagonPart, attacking_creature: Creature) -> Self {
        match (hexagon_part, attacking_creature.is_wide()) {
            (HexagonPart::Left, _)             => Self::Left,
            (HexagonPart::Right, _)            => Self::Right,
            (HexagonPart::TopHalfLeft,  false) => Self::TopLeft,
            (HexagonPart::TopHalfLeft,  true)  => Self::Top,
            (HexagonPart::TopHalfRight, false) => Self::TopRight,
            (HexagonPart::TopHalfRight, true)  => Self::Top,
            (HexagonPart::BotHalfLeft,  false) => Self::BottomLeft,
            (HexagonPart::BotHalfLeft,  true)  => Self::Bottom,
            (HexagonPart::BotHalfRight, false) => Self::BottomRight,
            (HexagonPart::BotHalfRight, true)  => Self::Bottom,
            (HexagonPart::BotLeft, _)          => Self::BottomLeft,
            (HexagonPart::BotRight, _)         => Self::BottomRight,
            (HexagonPart::TopLeft, _)          => Self::TopLeft,
            (HexagonPart::TopRight, _)         => Self::TopRight
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum HexagonPart {
    Left,
    TopLeft,
    TopHalfLeft,
    TopHalfRight,
    TopRight,
    Right,
    BotRight,
    BotHalfRight,
    BotHalfLeft,
    BotLeft
}

impl HexagonPart {
    // Конец дуги соответствующей части
    // если идти по часовой стрелке
    fn arc_end(&self) -> f32 {
        use std::f32::consts::*;
        // [0; 2*PI]
        // Ноль - середина левой стороны
        // Идём по часовой стрелке
        match self {
            Self::Left         => -(PI - FRAC_2_PI),
            Self::TopLeft      => -(FRAC_PI_2 + FRAC_2_PI),
            Self::TopHalfLeft  => -FRAC_PI_2,
            Self::TopHalfRight => -(FRAC_PI_2 - FRAC_2_PI),
            Self::TopRight     => -FRAC_2_PI,
            Self::Right        =>  FRAC_2_PI,
            Self::BotRight     =>  FRAC_PI_2 - FRAC_2_PI,
            Self::BotHalfRight =>  FRAC_PI_2,
            Self::BotHalfLeft  =>  FRAC_PI_2 + FRAC_2_PI,
            Self::BotLeft      =>  (PI - FRAC_2_PI)
        }
    }

    fn find_part_for_angle(angle: f32) -> Self {
        Self::iter()
            .find(|hex_part| angle < hex_part.arc_end())
            .unwrap_or(Self::Left)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridPos {
    pub x: i32,
    pub y: i32
}

impl GridPos {
    pub const X_RANGE: RangeInclusive<i32> = 1..=15;
    pub const Y_RANGE: RangeInclusive<i32> = 1..=11;

    pub const CELL_WIDTH: u32 = 46;
    pub const CELL_HEIGHT: u32 = 52;
    pub const CELL_VERTICAL: u32 = 32;

    const ODD_START_POINT: (i32, i32) = (105, 117);
    const EVEN_START_POINT: (i32, i32) = (83, 159);

    fn is_point_valid(x: i32, y: i32) -> bool {
        Self::X_RANGE.contains(&x) && Self::Y_RANGE.contains(&y)
    }

    pub fn new(x: i32, y: i32) -> Self {
        assert!(Self::is_point_valid(x, y));
        Self {x, y}
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

    pub fn center(&self) -> Point {
        let (x, y) = (self.x as u32 - 1, self.y as u32 - 1);

        // Вычитаем единицу чтобы рисовать клетки "внахлёст"
        let x_offset = x * (Self::CELL_WIDTH - 1);
        let y_offset = y / 2 * (Self::CELL_HEIGHT + Self::CELL_VERTICAL);

        let start_point = 
            if self.is_even_row() {
                Self::EVEN_START_POINT
            } else {
                Self::ODD_START_POINT
            };
        
        Point::from(start_point).offset(x_offset as i32, y_offset as i32)
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect::from_center(self.center(), Self::CELL_WIDTH, Self::CELL_HEIGHT)
    }

    pub fn find_pointer_position(point: Point) -> Option<GridPos> {
        let odd_x_relative = point.x() - (Self::ODD_START_POINT.0 - (Self::CELL_WIDTH / 2) as i32);
        let odd_y_relative = point.y() - (Self::ODD_START_POINT.1 - Self::CELL_HEIGHT as i32);
        let odd_x_rect = odd_x_relative / (Self::CELL_WIDTH - 1) as i32 + 1;
        let odd_y_rect = odd_y_relative / (Self::CELL_HEIGHT + Self::CELL_VERTICAL) as i32 + 1;

        let even_x_relative = point.x() - (Self::EVEN_START_POINT.0 - (Self::CELL_WIDTH / 2) as i32);
        let even_y_relative = point.y() - (Self::EVEN_START_POINT.1 - Self::CELL_HEIGHT as i32);
        let even_x_rect = even_x_relative / (Self::CELL_WIDTH - 1) as i32 + 1;
        let even_y_rect = even_y_relative / (Self::CELL_HEIGHT + Self::CELL_VERTICAL) as i32 + 1;

        let odd_y_rect_real = odd_y_rect * 2 - 1;
        let even_y_rect_real = even_y_rect * 2;

        let odd_cell = GridPos::try_new(odd_x_rect, odd_y_rect_real);
        let even_cell = GridPos::try_new(even_x_rect, even_y_rect_real);

        match (odd_cell, even_cell) {
            (None, None) => None,
            (val, None) | (None, val) => val.filter(|cell| cell.contains_point_precise(point)),
            (Some(cell_1), Some(cell_2)) => {
                match (cell_1.contains_point(point), cell_2.contains_point(point)) {
                    (false, false) => None,
                    (true, false) => Some(cell_1),
                    (false, true) => Some(cell_2),
                    (true, true) => {
                        let cell =
                            if cell_1.contains_point_precise(point) {
                                cell_1
                            } else {
                                cell_2
                            };
                        Some(cell)
                    }
                }
            }
        }
    }

    pub fn calculate_attack_direction(&self, point: Point, attacking_creature: Creature) -> AttackDirection {
        let grid_center = self.center();
        let point = point - grid_center;
        let x = point.x() as f32;
        let y = point.y() as f32;
        let angle = f32::atan2(y, x);
        let hexagon_part = HexagonPart::find_part_for_angle(angle);

        AttackDirection::from_hexagon_part(hexagon_part, attacking_creature) 
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.bounding_rect().contains_point(point)
    }

    fn contains_point_precise(&self, point: Point) -> bool {
        // http://www.playchilla.com/how-to-check-if-a-point-is-inside-a-hexagon
        let relative_point = point - self.center();
        let (abs_x, abs_y) = (relative_point.x().abs(), relative_point.y().abs());
        // -2 выбрано подбором
        let v = Self::CELL_VERTICAL as i32 / 2 - 2;
        let h = Self::CELL_HEIGHT as i32 / 2;
        let res = 2 * v * h - v * abs_x - h as i32 * abs_y;
        res > 0
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
