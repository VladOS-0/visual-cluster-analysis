use std::io::stdin;

use crate::{
    geometry::{Axis, Rectangle},
    visual::Image,
};

const DEFAULT_ELEMENTS_COUNT: usize = 5;

const MIN_POINTS_DISTANCE: f32 = 0.5;
const MAX_POINTS_DISTANCE: f32 = 5.0;

pub fn execute() {
    let elements_count = dialogue();

    let boundary = Rectangle::default();
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/hierarchy_grouping.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    //drawing.draw_axis(Axis::X, None, None);
    //drawing.draw_axis(Axis::Y, None, None);

    drawing.draw_axis(Axis::Other(Box::new(|x| Some(0.1 * x + 2.0))), None, None);

    drawing.save();
    drawing.show("gimp");

    println!("Границы: {} \n\n:", boundary);
}

fn dialogue() -> usize {
    let mut buf = String::new();

    println!(
        "Введите количество элементов (По умолчанию: {}).",
        DEFAULT_ELEMENTS_COUNT
    );
    stdin()
        .read_line(&mut buf)
        .expect("Не удалось прочитать из стандартного ввода.");
    let mut count = buf
        .trim()
        .parse::<usize>()
        .unwrap_or(DEFAULT_ELEMENTS_COUNT);
    if count == 0 {
        count = DEFAULT_ELEMENTS_COUNT;
    }

    count
}
