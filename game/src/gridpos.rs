use logic::grid::GridPos;
use sdl2::rect::{Point, Rect};

pub const CELL_WIDTH: i32 = 46;
pub const CELL_HEIGHT: i32 = 52;
pub const CELL_VERTICAL: i32 = 32;

const ODD_START_POINT: (i32, i32) = (105, 117);
const EVEN_START_POINT: (i32, i32) = (83, 159);

pub fn contains_point(pos: GridPos, point: Point) -> bool {
    bounding_rect(pos).contains_point(point)
}

pub fn bounding_rect(pos: GridPos) -> Rect {
    Rect::from_center(center(pos), CELL_WIDTH as u32, CELL_HEIGHT as u32)
}

pub fn center(pos: GridPos) -> Point {
    let (x, y) = (pos.x - 1, pos.y - 1);

    // Вычитаем единицу чтобы рисовать клетки "внахлёст"
    let x_offset = x * (CELL_WIDTH - 1);
    let y_offset = y / 2 * (CELL_HEIGHT + CELL_VERTICAL);

    let start_point = if pos.is_even_row() {
        EVEN_START_POINT
    } else {
        ODD_START_POINT
    };

    Point::from(start_point).offset(x_offset, y_offset)
}

pub fn find_pointer_position(point: Point) -> Option<GridPos> {
    let odd_x_relative = point.x() - (ODD_START_POINT.0 - (CELL_WIDTH / 2));
    let odd_y_relative = point.y() - (ODD_START_POINT.1 - CELL_HEIGHT);
    let odd_x_rect = odd_x_relative / (CELL_WIDTH - 1) + 1;
    let odd_y_rect = odd_y_relative / (CELL_HEIGHT + CELL_VERTICAL) + 1;

    let even_x_relative = point.x() - (EVEN_START_POINT.0 - (CELL_WIDTH / 2));
    let even_y_relative = point.y() - (EVEN_START_POINT.1 - CELL_HEIGHT);
    let even_x_rect = even_x_relative / (CELL_WIDTH - 1) + 1;
    let even_y_rect = even_y_relative / (CELL_HEIGHT + CELL_VERTICAL) + 1;

    let odd_y_rect_real = odd_y_rect * 2 - 1;
    let even_y_rect_real = even_y_rect * 2;

    let odd_cell = GridPos::try_new(odd_x_rect, odd_y_rect_real);
    let even_cell = GridPos::try_new(even_x_rect, even_y_rect_real);

    match (odd_cell, even_cell) {
        (None, None) => None,
        (val, None) | (None, val) => val.filter(|cell| contains_point_precise(*cell, point)),
        (Some(cell_1), Some(cell_2)) => {
            match (contains_point(cell_1, point), contains_point(cell_2, point)) {
                (false, false) => None,
                (true, false) => Some(cell_1),
                (false, true) => Some(cell_2),
                (true, true) => {
                    let cell = if contains_point_precise(cell_1, point) {
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

fn contains_point_precise(pos: GridPos, point: Point) -> bool {
    // http://www.playchilla.com/how-to-check-if-a-point-is-inside-a-hexagon
    let relative_point = point - center(pos);
    let (abs_x, abs_y) = (relative_point.x().abs(), relative_point.y().abs());
    // -2 выбрано подбором
    let v = CELL_VERTICAL / 2 - 2;
    let h = CELL_HEIGHT / 2;
    let res = 2 * v * h - v * abs_x - h * abs_y;
    res > 0
}
