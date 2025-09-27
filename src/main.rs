use crate::tasks::classification;

pub mod geometry;
pub mod tasks;
pub mod utils;
pub mod visual;

fn main() {
    classification::execute();
}
