use std::ops::Range;
use crate::grid::{Direction, Vec2Int};

pub fn get_rand_direction() -> Direction {
    match fastrand::i32(0..3) {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        _ => Direction::Up,
    }
}

pub fn get_rand_coord(x_range: Range<i32>, y_range: Range<i32>) -> Vec2Int {
    Vec2Int {x:fastrand::i32(x_range) , y: fastrand::i32(y_range)}
}
