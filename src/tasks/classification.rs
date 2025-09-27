use std::{collections::BTreeMap, io::stdin};

use crate::{
    geometry::{Point, Rectangular},
    visual::Image,
};

const DEFAULT_POINTS_COUNT: usize = 100;
const DEFAULT_CORES_COUNT: usize = 10;

pub fn execute() {
    let (points_count, cores_count) = dialogue();

    let boundary = Rectangular::default();
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/classification.png",
        boundary,
        true,
        None,
        None,
    );

    println!("Границы: {} \nЯдра:", boundary);

    let mut classes: BTreeMap<Point, Vec<Point>> = BTreeMap::new();

    for _ in 0..cores_count {
        let core = boundary.create_rand_point();
        classes.insert(core, Vec::new());
    }

    for _ in 0..points_count {
        let point = boundary.create_rand_point();
        let mut min_distance = f32::MAX;
        let mut class: Point = Point::new(f32::MAX, f32::MAX);
        for i in &mut classes {
            let distance = point.distance_to(*i.0);
            if distance < min_distance {
                class = *i.0;
                min_distance = distance;
            }
        }
        classes.get_mut(&class).unwrap().push(point);
    }

    let mut class_num: usize = 1;
    for class in &classes {
        println!("---------------------------------------");
        println!("{} класс: {}", class_num, class.0);
        let mut point_num = 1;
        drawing.draw_point_with_class(*class.0, class_num, true);
        for point in class.1 {
            drawing.draw_point_with_class(*point, class_num, false);
            println!(
                "{}: {} | Расстояние до центра класса: {}",
                point_num,
                point,
                point.distance_to(*class.0)
            );
            point_num += 1;
        }
        class_num += 1;
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
        "Введите количество ядер (По умолчанию: {}).",
        DEFAULT_CORES_COUNT
    );
    stdin()
        .read_line(&mut buf)
        .expect("Не удалось прочитать из стандартного ввода.");

    let mut cores_count = buf.trim().parse::<usize>().unwrap_or(DEFAULT_CORES_COUNT);
    if cores_count == 0 || cores_count >= points_count {
        cores_count = DEFAULT_CORES_COUNT;
    }
    println!("Выбранное количество ядер: {}\n", cores_count);

    (points_count, cores_count)
}
