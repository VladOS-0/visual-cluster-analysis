use std::process::exit;

use crate::utils::rand_f32_in_range;

const DEFAULT_BOTTOM_LEFT_X: f32 = 0.0;
const DEFAULT_BOTTOM_LEFT_Y: f32 = 0.0;
const DEFAULT_UPPER_RIGHT_X: f32 = 100.0;
const DEFAULT_UPPER_RIGHT_Y: f32 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn distance_to(self, other: Point) -> f32 {
        f32::sqrt(f32::powi(self.x - other.x, 2) + f32::powi(self.y - other.y, 2))
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}; {})", self.x, self.y).as_str())
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rectangular {
    pub bottom_left: Point,
    pub upper_right: Point,
}

impl Rectangular {
    pub fn new(bottom_left: Point, upper_right: Point) -> Self {
        if upper_right.x < bottom_left.x {
            eprintln!(
                "ОШИБКА: Попытка создать прямоугольник с отрциательной шириной ({}): Нижний левый угол - {}; Правый верхний угол - {}",
                upper_right.x - bottom_left.x,
                upper_right,
                bottom_left
            );
            exit(1);
        }
        if upper_right.y < bottom_left.y {
            eprintln!(
                "ОШИБКА: Попытка создать прямоугольник с отрциательной высотой ({}): Нижний левый угол - {}; Правый верхний угол - {}",
                upper_right.y - bottom_left.y,
                upper_right,
                bottom_left
            );
            exit(1);
        }
        Self {
            bottom_left,
            upper_right,
        }
    }
    pub fn center(&self) -> Point {
        Point::new(
            (self.upper_right.x - self.bottom_left.x) / 2.0,
            (self.upper_right.y - self.bottom_left.y) / 2.0,
        )
    }
    pub fn width(&self) -> f32 {
        self.upper_right.x - self.bottom_left.x
    }
    pub fn height(&self) -> f32 {
        self.upper_right.y - self.bottom_left.y
    }
    pub fn create_rand_point(&self) -> Point {
        Point::new(
            rand_f32_in_range(self.bottom_left.x, self.upper_right.x),
            rand_f32_in_range(self.bottom_left.y, self.upper_right.y),
        )
    }
}

impl std::fmt::Display for Rectangular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("От {} до {}", self.bottom_left, self.upper_right).as_str())
    }
}

impl Eq for Rectangular {}

impl Ord for Rectangular {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Default for Rectangular {
    fn default() -> Self {
        Self {
            bottom_left: Point::new(DEFAULT_BOTTOM_LEFT_X, DEFAULT_BOTTOM_LEFT_Y),
            upper_right: Point::new(DEFAULT_UPPER_RIGHT_X, DEFAULT_UPPER_RIGHT_Y),
        }
    }
}
