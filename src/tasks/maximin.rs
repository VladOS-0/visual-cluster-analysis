use std::{collections::HashMap, fmt::Display, io::stdin};

use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    geometry::{Axis, Point, Rectangle},
    visual::Image,
};

const DEFAULT_ELEMENTS_COUNT: usize = 400_000;

#[derive(Debug, Clone, PartialEq)]
struct Class {
    pub id: usize,
    pub core: ClassifiedPoint,
    pub max_distance_from_core: f32,
    pub farthest_element: usize,
    pub elements_count: usize,
}

impl Class {
    pub fn with_point(class_id: usize, point_id: usize, core: Point) -> Self {
        Self {
            id: class_id,
            core: ClassifiedPoint::new(point_id, class_id, true, core),
            max_distance_from_core: 0.0,
            farthest_element: point_id,
            elements_count: 0,
        }
    }
    pub fn with_classified_point(id: usize, core: &mut ClassifiedPoint) -> Self {
        core.class = id;
        core.is_core = true;
        Self {
            id,
            core: core.clone(),
            max_distance_from_core: 0.0,
            farthest_element: core.id,
            elements_count: 0,
        }
    }
    pub fn update_max_distance(&mut self, points: &HashMap<usize, ClassifiedPoint>) {
        self.max_distance_from_core = 0.0;
        self.farthest_element = self.core.id;

        for point in points.values().filter(|point| point.class == self.id) {
            let distance_to_core = self.core.inner.distance_to(&point.inner);
            if distance_to_core > self.max_distance_from_core {
                self.max_distance_from_core = distance_to_core;
                self.farthest_element = point.id;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ClassifiedPoint {
    pub id: usize,
    pub class: usize,
    pub is_core: bool,
    pub inner: Point,
}

impl ClassifiedPoint {
    pub fn new(id: usize, class: usize, is_core: bool, inner: Point) -> Self {
        Self {
            id,
            class,
            is_core,
            inner,
        }
    }
}

impl Display for ClassifiedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {}: {}, class {}", self.id, self.inner, self.class)
    }
}

pub fn execute() {
    let elements_count = dialogue();

    let boundary = Rectangle::new(Point::new(-100.0, -100.0), Point::new(100.0, 100.0));
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/maximin.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    drawing.draw_axis(Axis::X, None, None);
    drawing.draw_axis(Axis::Y, None, None);

    println!("Границы: {} \n\n", boundary);

    let first_class = Class::with_point(1, 1, boundary.create_rand_point());

    let mut classes: Vec<Class> = vec![first_class.clone()];

    let mut elements: HashMap<usize, ClassifiedPoint> = (2..=elements_count)
        .into_par_iter()
        .map(|point_id| {
            let new_point = boundary.create_rand_point();
            (
                point_id,
                ClassifiedPoint::new(point_id, 1, false, new_point),
            )
        })
        .collect();

    elements.insert(1, first_class.core.clone());

    let mut second_class_core_id = 1;
    let mut max_distance_from_first_class = 0.0;

    for element in elements.values() {
        if element.is_core {
            continue;
        }
        let distance_to_core = first_class.core.inner.distance_to(&element.inner);
        if distance_to_core > max_distance_from_first_class {
            max_distance_from_first_class = distance_to_core;
            second_class_core_id = element.id;
        }
    }

    if second_class_core_id == 1 {
        eprintln!("ERROR: Failed to find a core for the 2nd class");
        return;
    }

    let second_core = elements.get_mut(&second_class_core_id).unwrap();
    classes.push(Class::with_classified_point(2, second_core));

    loop {
        elements.par_iter_mut().for_each(|(_, element)| {
            if element.is_core {
                return;
            }
            let mut chosen_class: usize = 1;
            let mut lowest_distance = f32::MAX;

            for class in &classes {
                let distance_to_core = class.core.inner.distance_to(&element.inner);
                if distance_to_core < lowest_distance {
                    lowest_distance = distance_to_core;
                    chosen_class = class.id;
                }
            }
            element.class = chosen_class;
        });

        classes.par_iter_mut().for_each(|class| {
            class.update_max_distance(&elements);
        });

        let mut max_max_distance = 0.0;
        let mut farthest_farthest_element: usize = 0;

        for class in &classes {
            if class.max_distance_from_core > max_max_distance {
                max_max_distance = class.max_distance_from_core;
                farthest_farthest_element = class.farthest_element;
            }
        }

        if farthest_farthest_element == 0
            || elements.get(&farthest_farthest_element).unwrap().is_core
        {
            break;
        }

        let mut total_distance = 0.0;
        let mut pair_count = 0;

        for i in 0..classes.len() {
            for j in (i + 1)..classes.len() {
                total_distance += classes[i].core.inner.distance_to(&classes[j].core.inner);
                pair_count += 1;
            }
        }

        let average_distance_between_cores = if pair_count > 0 {
            total_distance / pair_count as f32
        } else {
            0.0
        };

        let threshold = average_distance_between_cores / 2.0;

        if max_max_distance > threshold {
            let new_class_id = classes.len() + 1;
            println!(
                "Создаем новый класс {} с ядром в точке {}",
                new_class_id, farthest_farthest_element
            );
            classes.push(Class::with_classified_point(
                new_class_id,
                elements.get_mut(&farthest_farthest_element).unwrap(),
            ));
        } else {
            println!(
                "Условие остановки: max_max_distance ({}) <= threshold ({})",
                max_max_distance, threshold
            );
            break;
        }
    }

    for element in elements.values() {
        if element.class > 0 && element.class <= classes.len() {
            classes[element.class - 1].elements_count += 1;
        }
    }

    for element in elements.values() {
        drawing.draw_point_with_class(element.inner, element.class, false, true);
    }

    println!("\n=== РЕЗУЛЬТАТЫ КЛАССИФИКАЦИИ ===");
    for class in &classes {
        println!(
            "Класс {}: {} элементов, ядро в точке {}",
            class.id, class.elements_count, class.core.id
        );
        println!("  Координаты ядра: {}", class.core.inner);
        drawing.draw_point_with_class(class.core.inner, class.id, true, true);
    }

    drawing.save();
    drawing.show("gimp");
}

fn dialogue() -> usize {
    let mut buf = String::new();

    println!(
        "Введите количество элементов (По умолчанию: {})",
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
