use std::io::stdin;

use crate::{
    geometry::{Axis, Rectangle},
    utils::{RoundToDecimalPlaces, rand_f32_in_range},
    visual::Image,
};

const DEFAULT_POINTS_COUNT: usize = 1000;

const MIN_FREE_COEFF: f32 = -20.0;
const MAX_FREE_COEFF: f32 = 20.0;

const MIN_COEFF: f32 = -5.0;
const MAX_COEFF: f32 = 5.0;

pub fn execute() {
    let points_count = dialogue();

    let boundary = Rectangle::default();
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/two_classes_function.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    drawing.draw_axis(Axis::X, None, None);
    drawing.draw_axis(Axis::Y, None, None);

    let w_0 = rand_f32_in_range(MIN_FREE_COEFF, MAX_FREE_COEFF, 2);
    let w_1 = rand_f32_in_range(MIN_COEFF, MAX_COEFF, 2);
    let w_2 = rand_f32_in_range(MIN_COEFF, MAX_COEFF, 2);

    let dividing_function = move |x: f32, y: f32| w_0 + w_1 * x + w_2 * y;

    drawing.draw_graph(&move |x: f32| Some((w_0 + w_1 * x) / -w_2), None);

    println!(
        "Границы: {} \n\nРазделяющая функция: f(x, y) = {} + {}x + {}y",
        boundary, w_0, w_1, w_2
    );
    println!("-------------------------");

    for i in 1..=points_count {
        let new_point = boundary.create_rand_point();
        let dividing_function_result = dividing_function(new_point.x, new_point.y);

        if dividing_function_result >= 0.0 {
            drawing.draw_point_with_class(new_point, 1, false, true);
            println!(
                "{} точка: {} | Значение разделяющей функции: {} | (I класс)",
                i,
                new_point,
                dividing_function_result.round_to_dp(2)
            );
        } else {
            drawing.draw_point_with_class(new_point, 2, false, true);
            println!(
                "{} точка: {} | Значение разделяющей функции: {} | (II класс)",
                i,
                new_point,
                dividing_function_result.round_to_dp(2)
            );
        }
    }

    println!("-------------------------");
    println!(
        "Разделяющая функция: f(x, y) = {} + {}x + {}y",
        w_0, w_1, w_2
    );

    drawing.save();
    drawing.show("gimp");
}

fn dialogue() -> usize {
    let mut buf = String::new();

    println!(
        "Введите количество точек (По умолчанию: {}).",
        DEFAULT_POINTS_COUNT
    );
    stdin()
        .read_line(&mut buf)
        .expect("Не удалось прочитать из стандартного ввода.");
    let mut points_count = buf.trim().parse::<usize>().unwrap_or(DEFAULT_POINTS_COUNT);
    if points_count == 0 {
        points_count = DEFAULT_POINTS_COUNT;
    }
    println!("Выбранное количество точек: {}", points_count);
    buf.clear();

    points_count
}
