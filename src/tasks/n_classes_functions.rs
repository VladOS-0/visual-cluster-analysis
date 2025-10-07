use std::io::stdin;

use crate::{geometry::Rectangular, utils::rand_f32_in_range, visual::Image};

const DEFAULT_POINTS_COUNT: usize = 1000;
const DEFAULT_CLASSES_COUNT: usize = 5;

const MIN_FREE_COEFF: f32 = -20.0;
const MAX_FREE_COEFF: f32 = 20.0;

const MIN_COEFF: f32 = -3.0;
const MAX_COEFF: f32 = 3.0;

pub fn execute() {
    let (points_count, classes_count) = dialogue();

    let boundary = Rectangular::default();
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/n_classes_functions.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    drawing.draw_axises(None);

    println!("Границы: {} \n\nРазделяющие функции:", boundary);

    let mut classes: Vec<(f32, f32, f32)> = Vec::with_capacity(classes_count);

    for i in 1..=classes_count {
        let w_0 = rand_f32_in_range(MIN_FREE_COEFF, MAX_FREE_COEFF, 2);
        let w_1 = rand_f32_in_range(MIN_COEFF, MAX_COEFF, 2);
        let w_2 = rand_f32_in_range(MIN_COEFF, MAX_COEFF, 2);

        println!("{}. f(x, y) = {} + {}x + {}y", i, w_0, w_1, w_2);

        drawing.draw_graph(move |x: f32| Some((w_0 + w_1 * x) / -w_2), None);
        classes.push((w_0, w_1, w_2));
    }
    println!("-------------------------");

    for i in 1..=points_count {
        let new_point = boundary.create_rand_point();
        println!("№{} {}", i, new_point);

        let mut chosen_class: usize = 0;
        let mut highest_score = f32::MIN;

        for j in 1..=classes.len() {
            let dividing_function_result =
                classes[j - 1].0 + classes[j - 1].1 * new_point.x + classes[j - 1].2 * new_point.y;
            println!(
                "{}. f(x, y) = {} + {} * {} + {} * {} = {}",
                j,
                classes[j - 1].0,
                classes[j - 1].1,
                new_point.x,
                classes[j - 1].2,
                new_point.y,
                dividing_function_result
            );
            if dividing_function_result > highest_score {
                highest_score = dividing_function_result;
                chosen_class = j;
            }
        }
        println!("Присвоен класс - {}", chosen_class);
        drawing.draw_point_with_class(new_point, chosen_class, false, true);
    }

    println!("-------------------------\nРазделяющие функции:");
    for (i, (w_0, w_1, w_2)) in classes.iter().enumerate() {
        println!("{}. f(x, y) = {} + {}x + {}y", i, w_0, w_1, w_2);
    }

    drawing.save();
    drawing.show("gimp");
}

fn dialogue() -> (usize, usize) {
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

    println!(
        "Введите количество классов (По умолчанию: {}).",
        DEFAULT_CLASSES_COUNT
    );
    stdin()
        .read_line(&mut buf)
        .expect("Не удалось прочитать из стандартного ввода.");
    let mut classes_count = buf.trim().parse::<usize>().unwrap_or(DEFAULT_CLASSES_COUNT);
    if classes_count == 0 {
        classes_count = DEFAULT_CLASSES_COUNT;
    }
    println!("Выбранное количество классов: {}", classes_count);
    buf.clear();

    (points_count, classes_count)
}
