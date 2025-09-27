use std::io::stdin;

use crate::geometry::{Point, Rectangular};

const DEFAULT_POINT_AMOUNT: usize = 100;

const BOTTOM_LEFT_X: f32 = 0.0;
const BOTTOM_LEFT_Y: f32 = 0.0;
const UPPER_RIGHT_X: f32 = 100.0;
const UPPER_RIGHT_Y: f32 = 100.0;

pub fn execute() {
    let point_amount = dialogue();

    let bottom_left = Point::new(BOTTOM_LEFT_X, BOTTOM_LEFT_Y);
    let upper_right = Point::new(UPPER_RIGHT_X, UPPER_RIGHT_Y);

    let boundary = Rectangular::new(bottom_left, upper_right);

    let center = boundary.center();
    println!("Границы:{}\nЦентр - {}\nТочки:", boundary, center);

    let mut class_center: Option<Point> = None;
    let mut min_distance: Option<f32> = None;

    for i in 1..point_amount {
        let point = boundary.create_rand_point();
        let distance = point.distance_to(center);
        if min_distance.is_none() || min_distance.unwrap() > distance {
            class_center = Some(point);
            min_distance = Some(distance);
        }
        println!("{}: {} | Расстояние до центра: {}", i, point, distance);
    }

    if class_center.is_none() || min_distance.is_none() {
        panic!("У нас буквально нету победителя!! КОШМАР!!")
    }

    println!(
        "\nЦЕНТР КЛАССА - {} с расстоянием до центра {}",
        class_center.unwrap(),
        min_distance.unwrap()
    );
}

fn dialogue() -> usize {
    let mut buf = String::new();

    println!(
        "Введите количество точек (По умолчанию: {}).",
        DEFAULT_POINT_AMOUNT
    );
    stdin()
        .read_line(&mut buf)
        .expect("Не удалось прочитать из стандартного ввода.");
    let mut count = buf.parse::<usize>().unwrap_or(DEFAULT_POINT_AMOUNT);
    if count == 0 {
        count = DEFAULT_POINT_AMOUNT;
    }

    count
}
