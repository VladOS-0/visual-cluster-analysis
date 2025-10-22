use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    io::stdin,
};

use crate::{
    geometry::{Axis, Point, Rectangle},
    utils::{rand_f32_in_range, rand_f32_in_range_with_distance},
    visual::Image,
};

type ElementId = usize;

const DEFAULT_ELEMENTS_COUNT: usize = 5;

const MIN_POINTS_DISTANCE: f32 = 0.5;
const DISTANCE_BETWEEN_VALUES: f32 = 0.5;
const MAX_POINTS_DISTANCE: f32 = 5.0;

#[derive(Clone, Debug)]
struct Hierarchy {
    pub objects: BTreeMap<ElementId, HierarchyObject>,
    pub element_count: usize,
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
                    write!(f, "{}    ", distance)?;
                } else {
                    write!(f, "{}  ", distance)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Hierarchy {
    fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
            element_count: 0,
        }
    }

    fn populate(&mut self, count: usize) {
        self.element_count = count;
        for i in 1..=count {
            self.objects.insert(i, HierarchyObject::leaf(i));
        }
    }

    fn init_rand_distances(&mut self) {
        let keys: Vec<ElementId> = self.objects.keys().map(|id_ref| *id_ref).collect();
        let mut cached_distances: HashMap<(ElementId, ElementId), f32> = HashMap::new();

        for i in 1..=self.objects.len() {
            let obj = self.objects.get_mut(&i).unwrap();
            obj.init_rand_distances(&keys, &mut cached_distances);
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

            self.element_count += 1;
            let mut new_node =
                HierarchyObject::fold(self.element_count, first_pair_obj, second_pair_obj);

            for object in self.objects.values_mut() {
                object.distances.remove(&current_pair.0);
                object.distances.remove(&current_pair.1);
            }
            new_node.calculate_all_distances(&mut self.objects);

            self.objects.insert(self.element_count, new_node);
        }
    }
}

#[derive(Clone, Debug)]
struct HierarchyObject {
    pub id: ElementId,
    pub inner: InnerHierarchyObject,
    pub distances: BTreeMap<ElementId, f32>,
}
impl HierarchyObject {
    fn leaf(id: ElementId) -> Self {
        Self {
            id,
            inner: InnerHierarchyObject::Leaf,
            distances: BTreeMap::new(),
        }
    }
    fn init_rand_distances(
        &mut self,
        elements: &Vec<ElementId>,
        cached_distances: &mut HashMap<(ElementId, ElementId), f32>,
    ) {
        for id in elements {
            if *id == self.id {
                self.distances.insert(*id, 0.0);
            } else if let Some(cached_distance) = cached_distances.get(&(*id, self.id)) {
                self.distances.insert(*id, *cached_distance);
            } else {
                let random_distance = rand_f32_in_range_with_distance(
                    MIN_POINTS_DISTANCE,
                    MAX_POINTS_DISTANCE,
                    DISTANCE_BETWEEN_VALUES,
                    1,
                );
                cached_distances.insert((self.id, *id), random_distance);
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
                    "УЗЕЛ: {} - Расстояние: {}; Составляющие: {} {} - {} {}",
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
}

#[derive(Clone, Debug)]
enum InnerHierarchyObject {
    Node(Box<(HierarchyObject, HierarchyObject, f32)>),
    Leaf,
}

pub fn execute() {
    let elements_count = dialogue();

    let boundary = Rectangle::new(Point::new(-10.0, -10.0), Point::new(190.0, 190.0));
    let mut drawing = Image::new(
        "/home/vlad0s/Изображения/Misc/labs/hierarchy_grouping.png",
        boundary.clone(),
        true,
        None,
        None,
    );

    drawing.draw_axis(Axis::X, None, None);
    drawing.draw_axis(Axis::Y, None, None);

    println!("Границы: {} \n\n:", boundary);

    let mut hierarchy = Hierarchy::new();
    hierarchy.populate(elements_count);
    hierarchy.init_rand_distances();

    println!("Исходные расстояния:");
    println!("{}", hierarchy);

    hierarchy.assemble();

    println!("Получившаяся иерархия: ");
    hierarchy
        .objects
        .first_entry()
        .unwrap()
        .get()
        .tree_display();

    drawing.save();
    //drawing.show("gimp");
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
