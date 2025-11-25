use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    io::stdin,
};

use crate::{
    geometry::{Axis, Point, Rectangle},
    random::Random,
    utils::RoundToDecimalPlaces,
    visual::{Color, Image},
};

type ElementId = usize;

const DEFAULT_ELEMENTS_COUNT: usize = 5;

const MIN_POINTS_DISTANCE: f32 = 0.4;
const DISTANCE_BETWEEN_VALUES: f32 = 0.2;
const MAX_POINTS_DISTANCE: f32 = 10.0;

const REVERSE_MIN_POINTS_DISTANCE: f32 = 0.2;
const REVERSE_DISTANCE_BETWEEN_VALUES: f32 = 0.1;
const REVERSE_MAX_POINTS_DISTANCE: f32 = 1.5;

const VISUAL_POINTS_DISTANCE: f32 = 40.0;

/// На графике по оси Y точки будут располагаться на расстоянии MIN_POINTS_DISTANCE * VISUAL_DISTANCE_MULTIPLIER
const VISUAL_DISTANCE_MULTIPLIER: f32 = 40.0;
const REVERSE_VISUAL_DISTANCE_MULTIPLIER: f32 = 100.0;

const BIAS_PROB: f32 = 0.7;

const BIAS_MULT: f32 = 3.0;
const BIAS_START: f32 = MIN_POINTS_DISTANCE;
const BIAS_END: f32 = MIN_POINTS_DISTANCE + (MAX_POINTS_DISTANCE - MIN_POINTS_DISTANCE) / 2.0;

const REVERSE_BIAS_MULT: f32 = 0.2;
const REVERSE_BIAS_START: f32 =
    REVERSE_MIN_POINTS_DISTANCE + (REVERSE_MAX_POINTS_DISTANCE - REVERSE_MIN_POINTS_DISTANCE) / 3.0;
const REVERSE_BIAS_END: f32 = REVERSE_MAX_POINTS_DISTANCE;

const CACHE_REGENERATE_ATTEMPTS: usize = 5;

#[derive(Clone, Debug)]
struct Hierarchy {
    pub objects: BTreeMap<ElementId, HierarchyObject>,
    pub max_element_id: usize,
    pub points_count: usize,
    pub reverse: bool,
}

impl Display for Hierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for id in self.objects.keys() {
            if id / 10 < 1 {
                write!(f, "{}    ", id)?;
            } else {
                write!(f, "{}   ", id)?;
            }
        }
        write!(f, "\n")?;
        for (id, object) in &self.objects {
            if id / 10 < 1 {
                write!(f, "{}  ", id)?;
            } else {
                write!(f, "{} ", id)?;
            }
            for distance in object.distances.values() {
                if *distance == distance.round() {
                    if *distance == 0.0 {
                        write!(f, "-    ")?;
                    }
                    write!(f, "{}    ", distance)?;
                } else {
                    write!(f, "{}  ", distance)?;
                }
            }
            write!(f, "\n")?;
        }

        if self.reverse {
            writeln!(f, "\n\n Значения до получения обратных:")?;
            write!(f, "   ")?;
            for id in self.objects.keys() {
                if id / 10 < 1 {
                    write!(f, "{}    ", id)?;
                } else {
                    write!(f, "{}   ", id)?;
                }
            }
            write!(f, "\n")?;
            for (id, object) in &self.objects {
                if id / 10 < 1 {
                    write!(f, "{}  ", id)?;
                } else {
                    write!(f, "{} ", id)?;
                }
                for distance in object
                    .distances
                    .values()
                    .map(|distance| (1.0 / *distance).round_to_dp(1))
                {
                    if distance == distance.round() {
                        if distance == f32::INFINITY {
                            write!(f, "-    ")?;
                        } else {
                            write!(f, "{}    ", distance)?;
                        }
                    } else {
                        write!(f, "{}  ", distance)?;
                    }
                }
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl Hierarchy {
    fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
            max_element_id: 0,
            points_count: 0,
            reverse: false,
        }
    }

    fn reverse(self) -> Self {
        debug_assert!(
            self.objects.is_empty(),
            "reverse() не модифицирует элементы иерархии, а просто помечает её так для будущей генерации. У переданной же иерархии уже есть {} элементов",
            self.objects.len()
        );

        Self {
            reverse: !self.reverse,
            ..self
        }
    }

    fn populate(&mut self, count: usize) {
        self.max_element_id = count;
        self.points_count = count;

        for i in 1..=count {
            self.objects.insert(i, HierarchyObject::leaf(i));
        }
    }

    fn init_rand_distances(&mut self, mut image: Option<&mut Image>) {
        let keys: Vec<ElementId> = self.objects.keys().map(|id_ref| *id_ref).collect();
        let mut cached_pairs: HashMap<(ElementId, ElementId), f32> = HashMap::new();
        let mut cached_distances: Vec<f32> = Vec::new();

        for i in 1..=self.objects.len() {
            let obj = self.objects.get_mut(&i).unwrap();
            obj.init_rand_distances(
                &keys,
                &mut cached_pairs,
                &mut cached_distances,
                self.reverse,
            );

            if image.is_some() {
                let image = image.as_deref_mut().unwrap();
                let point_position = Point::new(i as f32 * VISUAL_POINTS_DISTANCE + 10.0, 0.0);

                // нарисовать засечку
                image.draw_line(
                    point_position.y_offset(-3.0),
                    point_position.y_offset(3.0),
                    None,
                );

                image.write(point_position.offset(-4.0, -8.0), format!("X{}", i), None);
            }
        }

        if image.is_some() {
            let image = image.as_deref_mut().unwrap();
            for distance in cached_distances {
                let distance_point_position = Point::new(
                    0.0,
                    distance
                        * if self.reverse {
                            REVERSE_VISUAL_DISTANCE_MULTIPLIER
                        } else {
                            VISUAL_DISTANCE_MULTIPLIER
                        },
                );

                // нарисовать засечку
                image.draw_line(
                    distance_point_position.x_offset(-3.0),
                    distance_point_position.x_offset(3.0),
                    None,
                );

                image.write(
                    distance_point_position.offset(-25.0, -2.0),
                    distance.to_string(),
                    None,
                );

                if self.reverse {
                    image.write(
                        distance_point_position.offset(3.0, -2.0),
                        (1.0 / distance).round_to_dp(1).to_string(),
                        Some(Color::hex("#8a2d1e", 1.0)),
                    );
                }
            }
        }
    }

    fn assemble(&mut self) {
        while self.objects.len() > 1 {
            let mut min_distance = f32::MAX;
            let mut current_pair: (ElementId, ElementId) = (0, 0);

            for obj in &self.objects {
                for (id_second, distance) in &obj.1.distances {
                    if *distance < min_distance && *id_second != *obj.0 {
                        min_distance = *distance;
                        current_pair = (*obj.0, *id_second);
                    }
                }
            }

            let first_pair_obj = self.objects.remove(&current_pair.0).unwrap();
            let second_pair_obj = self.objects.remove(&current_pair.1).unwrap();

            self.max_element_id += 1;
            let mut new_node =
                HierarchyObject::fold(self.max_element_id, first_pair_obj, second_pair_obj);

            for object in self.objects.values_mut() {
                object.distances.remove(&current_pair.0);
                object.distances.remove(&current_pair.1);
            }
            new_node.calculate_all_distances(&mut self.objects);

            self.objects.insert(self.max_element_id, new_node);
        }
    }

    fn tree_display(&self) {
        if self.objects.len() != 1 {
            panic!(
                "Hierarchy has {} top-level elements instead of 1! It is empty or not yet assembled!",
                self.objects.len()
            );
        }
        self.objects.first_key_value().unwrap().1.tree_display();
    }

    fn draw(&self, drawing: &mut Image) {
        if self.objects.len() != 1 {
            panic!(
                "Hierarchy has {} top-level elements instead of 1! It is empty or not yet assembled!",
                self.objects.len()
            );
        }
        //let mut drawing_order
        self.objects
            .first_key_value()
            .unwrap()
            .1
            .draw(drawing, self.reverse);
    }

    fn assemble_drawing_queue(&self) -> BTreeMap<f32, ()> {
        BTreeMap::new()
    }
}

#[derive(Clone, Debug)]
struct HierarchyObject {
    pub id: ElementId,
    pub inner: InnerHierarchyObject,
    pub distances: BTreeMap<ElementId, f32>,
    pub self_distance: Option<f32>,
}
impl HierarchyObject {
    fn leaf(id: ElementId) -> Self {
        Self {
            id,
            inner: InnerHierarchyObject::Leaf,
            distances: BTreeMap::new(),
            self_distance: None,
        }
    }
    fn pretty_id(&self, points_count: usize) -> usize {
        match self.inner {
            InnerHierarchyObject::Node(_) => self.id - points_count,
            InnerHierarchyObject::Leaf => self.id,
        }
    }

    fn init_rand_distances(
        &mut self,
        elements: &Vec<ElementId>,
        cached_pairs: &mut HashMap<(ElementId, ElementId), f32>,
        cached_distances: &mut Vec<f32>,
        reverse: bool,
    ) {
        for id in elements {
            if *id == self.id {
                self.distances.insert(*id, 0.0);
            } else if let Some(cached_distance) = cached_pairs.get(&(*id, self.id)) {
                self.distances.insert(*id, *cached_distance);
            } else {
                let random_distance = if reverse {
                    (1.0 / Random::new()
                        .range(REVERSE_MIN_POINTS_DISTANCE, REVERSE_MAX_POINTS_DISTANCE)
                        .distance(REVERSE_DISTANCE_BETWEEN_VALUES)
                        .bias(
                            REVERSE_BIAS_MULT,
                            BIAS_PROB,
                            Some(REVERSE_BIAS_START),
                            Some(REVERSE_BIAS_END),
                        )
                        .cache(&cached_distances, CACHE_REGENERATE_ATTEMPTS)
                        .to_dp(1)
                        .generate())
                    .round_to_dp(1)
                } else {
                    Random::new()
                        .range(MIN_POINTS_DISTANCE, MAX_POINTS_DISTANCE)
                        .distance(DISTANCE_BETWEEN_VALUES)
                        .bias(BIAS_MULT, BIAS_PROB, Some(BIAS_START), Some(BIAS_END))
                        .cache(&cached_distances, CACHE_REGENERATE_ATTEMPTS)
                        .to_dp(1)
                        .generate()
                };

                cached_pairs.insert((self.id, *id), random_distance);

                if cached_distances
                    .iter()
                    .find(|cached| **cached == random_distance)
                    .is_none()
                {
                    cached_distances.push(random_distance);
                }

                self.distances.insert(*id, random_distance);
            }
        }
    }

    fn calculate_all_distances(&mut self, objects: &mut BTreeMap<ElementId, HierarchyObject>) {
        for object in objects.values_mut() {
            let distance = self.calculate_distance_to(object);
            self.distances.insert(object.id, distance);
            object.distances.insert(self.id, distance);
        }
    }

    fn calculate_distance_to(&self, other: &HierarchyObject) -> f32 {
        match &self.inner {
            InnerHierarchyObject::Node(pair) => pair
                .0
                .distances
                .get(&other.id)
                .unwrap()
                .min(*pair.1.distances.get(&other.id).unwrap()),
            InnerHierarchyObject::Leaf => *other.distances.get(&self.id).unwrap(),
        }
    }

    fn fold(id: ElementId, first: HierarchyObject, second: HierarchyObject) -> Self {
        let distance_between = *first.distances.get(&second.id).unwrap();
        Self {
            id,
            inner: InnerHierarchyObject::Node(Box::new((first, second, distance_between))),
            distances: BTreeMap::new(),
            self_distance: Some(distance_between),
        }
    }

    fn tree_display(&self) {
        match &self.inner {
            InnerHierarchyObject::Node(node) => {
                let first_member_prefix = match node.0.inner {
                    InnerHierarchyObject::Node(_) => "УЗЕЛ",
                    InnerHierarchyObject::Leaf => "ЛИСТ",
                };
                let second_member_prefix = match node.1.inner {
                    InnerHierarchyObject::Node(_) => "УЗЕЛ",
                    InnerHierarchyObject::Leaf => "ЛИСТ",
                };
                println!(
                    "УЗЕЛ: {} - Расстояние: {}; Составляющие: {} {} + {} {}",
                    self.id,
                    node.2,
                    first_member_prefix,
                    node.0.id,
                    second_member_prefix,
                    node.1.id
                );
                node.0.tree_display();
                node.1.tree_display();
            }
            InnerHierarchyObject::Leaf => {}
        }
    }

    fn draw(&self, drawing: &mut Image, reverse: bool) -> Point {
        let color = Color::rand();

        let (first_leg, second_leg) = match &self.inner {
            InnerHierarchyObject::Node(node) => {
                (node.0.draw(drawing, reverse), node.1.draw(drawing, reverse))
            }
            InnerHierarchyObject::Leaf => {
                return Point::new(self.id as f32 * VISUAL_POINTS_DISTANCE + 10.0, 0.0);
            }
        };

        drawing.draw_polyline(
            vec![
                first_leg,
                Point::new(
                    first_leg.x,
                    self.self_distance.unwrap()
                        * if reverse {
                            REVERSE_VISUAL_DISTANCE_MULTIPLIER
                        } else {
                            VISUAL_DISTANCE_MULTIPLIER
                        },
                ),
                Point::new(
                    second_leg.x,
                    self.self_distance.unwrap()
                        * if reverse {
                            REVERSE_VISUAL_DISTANCE_MULTIPLIER
                        } else {
                            VISUAL_DISTANCE_MULTIPLIER
                        },
                ),
                second_leg,
            ],
            Some(color),
        );

        let hierarchy_center = Point::new(
            first_leg.x
                + (second_leg.x - first_leg.x) / Random::new().range(1.4, 2.8).to_dp(3).generate(),
            self.self_distance.unwrap()
                * if reverse {
                    REVERSE_VISUAL_DISTANCE_MULTIPLIER
                } else {
                    VISUAL_DISTANCE_MULTIPLIER
                },
        );

        // draw hierarchy id
        drawing.write(
            hierarchy_center.offset(-3.0, -5.0),
            self.id.to_string(),
            Some(color),
        );

        hierarchy_center
    }
}

#[derive(Clone, Debug)]
enum InnerHierarchyObject {
    Node(Box<(HierarchyObject, HierarchyObject, f32)>),
    Leaf,
}

pub fn execute() {
    let (elements_count, reverse) = dialogue();

    let boundary = Rectangle::new(
        Point::new(-30.0, -10.0),
        Point::new(50.0 + elements_count as f32 * 50.0, 550.0),
    );
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/hierarchy_grouping.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    drawing.draw_axis(Axis::X, None, None);
    drawing.draw_axis(Axis::Y, None, None);

    println!("Границы: {} \n\n", boundary);

    let mut hierarchy = if reverse {
        Hierarchy::new().reverse()
    } else {
        Hierarchy::new()
    };

    hierarchy.populate(elements_count);
    hierarchy.init_rand_distances(Some(&mut drawing));

    println!("Сгенерированные расстояния:");
    println!("{}", hierarchy);

    hierarchy.assemble();

    println!("Получившаяся иерархия: ");
    hierarchy.tree_display();

    hierarchy.draw(&mut drawing);

    drawing.save();
    drawing.show("gimp");
}

fn dialogue() -> (usize, bool) {
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

    buf.clear();
    println!("Должна ли иерархия собираться по максимальному (1) или минимальному (0) расстоянию?");
    stdin()
        .read_line(&mut buf)
        .expect("Не удалось прочитать из стандартного ввода.");
    let reverse = buf.trim().parse::<usize>().unwrap_or(0) == 1;

    (count, reverse)
}
