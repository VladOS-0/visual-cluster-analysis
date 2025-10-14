use std::process::exit;

use crate::utils::rand_f32_in_range;

const DEFAULT_BOTTOM_LEFT_X: f32 = -100.0;
const DEFAULT_BOTTOM_LEFT_Y: f32 = -100.0;
const DEFAULT_TOP_RIGHT_X: f32 = 100.0;
const DEFAULT_TOP_RIGHT_Y: f32 = 100.0;

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

pub enum Axis {
    X,
    Y,
    Other(Box<dyn Fn(f32) -> Option<f32>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixedPosition {
    BottomLeft(f32, f32),
    MiddleLeft(f32, f32),
    TopLeft(f32, f32),
    TopMiddle(f32, f32),
    TopRight(f32, f32),
    MiddleRight(f32, f32),
    BottomRight(f32, f32),
    BottomMiddle(f32, f32),
    Center(f32, f32),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rectangle {
    pub bottom_left: Point,
    pub top_right: Point,
}

impl Rectangle {
    pub fn new(bottom_left: Point, top_right: Point) -> Self {
        if top_right.x < bottom_left.x {
            eprintln!(
                "ОШИБКА: Попытка создать прямоугольник с отрциательной шириной ({}): Нижний левый угол - {}; Правый верхний угол - {}",
                top_right.x - bottom_left.x,
                top_right,
                bottom_left
            );
            exit(1);
        }
        if top_right.y < bottom_left.y {
            eprintln!(
                "ОШИБКА: Попытка создать прямоугольник с отрциательной высотой ({}): Нижний левый угол - {}; Правый верхний угол - {}",
                top_right.y - bottom_left.y,
                top_right,
                bottom_left
            );
            exit(1);
        }
        Self {
            bottom_left,
            top_right,
        }
    }
    pub fn get_position(&self, position: FixedPosition) -> Point {
        let x: f32;
        let y: f32;

        match position {
            FixedPosition::BottomLeft(offset_x, offset_y) => {
                x = (self.bottom_left.x + offset_x).clamp(self.bottom_left.x, self.top_right.x);
                y = (self.bottom_left.y + offset_y).clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::MiddleLeft(offset_x, offset_y) => {
                x = (self.bottom_left.x + offset_x).clamp(self.bottom_left.x, self.top_right.x);
                y = (self.bottom_left.y + self.height() / 2.0 + offset_y)
                    .clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::TopLeft(offset_x, offset_y) => {
                x = (self.bottom_left.x + offset_x).clamp(self.bottom_left.x, self.top_right.x);
                y = (self.top_right.y + offset_y).clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::TopMiddle(offset_x, offset_y) => {
                x = (self.bottom_left.x + self.width() / 2.0 + offset_x)
                    .clamp(self.bottom_left.x, self.top_right.x);
                y = (self.top_right.y + offset_y).clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::TopRight(offset_x, offset_y) => {
                x = (self.top_right.x + offset_x).clamp(self.bottom_left.x, self.top_right.x);
                y = (self.top_right.y + offset_y).clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::MiddleRight(offset_x, offset_y) => {
                x = (self.top_right.x + offset_x).clamp(self.bottom_left.x, self.top_right.x);
                y = (self.bottom_left.y + self.height() / 2.0 + offset_y)
                    .clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::BottomRight(offset_x, offset_y) => {
                x = (self.bottom_left.x + offset_x).clamp(self.bottom_left.x, self.top_right.x);
                y = (self.top_right.y + offset_y).clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::BottomMiddle(offset_x, offset_y) => {
                x = (self.bottom_left.x + self.width() / 2.0 + offset_x)
                    .clamp(self.bottom_left.x, self.top_right.x);
                y = (self.bottom_left.y + offset_y).clamp(self.bottom_left.y, self.top_right.y);
            }
            FixedPosition::Center(offset_x, offset_y) => {
                x = (self.bottom_left.x + self.width() / 2.0 + offset_x)
                    .clamp(self.bottom_left.x, self.top_right.x);
                y = (self.bottom_left.y + self.height() / 2.0 + offset_y)
                    .clamp(self.bottom_left.y, self.top_right.y);
            }
        }
        Point::new(x, y)
    }

    pub fn center(&self) -> Point {
        self.get_position(FixedPosition::Center(0.0, 0.0))
    }
    pub fn width(&self) -> f32 {
        self.top_right.x - self.bottom_left.x
    }
    pub fn height(&self) -> f32 {
        self.top_right.y - self.bottom_left.y
    }
    pub fn create_rand_point(&self) -> Point {
        Point::new(
            rand_f32_in_range(self.bottom_left.x, self.top_right.x, 0),
            rand_f32_in_range(self.bottom_left.y, self.top_right.y, 0),
        )
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("От {} до {}", self.bottom_left, self.top_right).as_str())
    }
}

impl Eq for Rectangle {}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            bottom_left: Point::new(DEFAULT_BOTTOM_LEFT_X, DEFAULT_BOTTOM_LEFT_Y),
            top_right: Point::new(DEFAULT_TOP_RIGHT_X, DEFAULT_TOP_RIGHT_Y),
        }
    }
}
