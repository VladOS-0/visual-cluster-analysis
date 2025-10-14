use std::io::stdin;

use crate::{
    geometry::{Point, Rectangle},
    utils::RoundToDecimalPlaces,
    visual::Image,
};

const DEFAULT_POINT_AMOUNT: usize = 1000;

pub fn execute() {
    let point_amount = dialogue();

    let boundary = Rectangle::default();

    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/k_mean.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    let center = boundary.center();
    println!("Границы:{}\nЦентр - {}\nТочки:", boundary, center);

    let mut class_center: Option<Point> = None;
    let mut min_distance: Option<f32> = None;

    for i in 1..point_amount {
        let point = boundary.create_rand_point();
        drawing.draw_point_with_class(point, 1, false, false);

        let distance = point.distance_to(center);
        if min_distance.is_none() || min_distance.unwrap() > distance {
            class_center = Some(point);
            min_distance = Some(distance);
        }
        println!(
            "{}: {} | Расстояние до центра: {}",
            i,
            point,
            distance.round_to_dp(2)
        );
    }

    if class_center.is_none() || min_distance.is_none() {
        panic!("У нас буквально нету победителя!! КОШМАР!!")
    }
    let class_center = class_center.unwrap();
    let min_distance = min_distance.unwrap();

    drawing.draw_point_with_class(class_center, 1, true, false);

    println!(
        "\nЦЕНТР КЛАССА - {} с расстоянием до центра {}",
        class_center, min_distance
    );

    drawing.save();
    drawing.show("gimp");
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
    let mut count = buf.trim().parse::<usize>().unwrap_or(DEFAULT_POINT_AMOUNT);
    if count == 0 {
        count = DEFAULT_POINT_AMOUNT;
    }

    count
}
