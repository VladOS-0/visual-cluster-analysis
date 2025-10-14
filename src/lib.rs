use std::{io::stdin, process::exit};

use crate::tasks::{
    classification, hierarchy_grouping, k_mean, n_classes_functions, two_classes_function,
};

#[allow(non_upper_case_globals)]
pub mod font;
pub mod geometry;
pub mod tasks;
pub mod utils;
pub mod visual;

pub fn interactive() {
    loop {
        println!("Какое задание выполнить? (1-5, 0 для выхода)");
        let mut buf = String::new();
        stdin()
            .read_line(&mut buf)
            .expect("Не удалось прочитать из стандартного ввода.");
        let index = buf.trim().parse::<usize>();
        if let Ok(index) = index {
            match index {
                0 => {
                    println!("Работа программы завершена");
                    exit(0);
                }
                1 => {
                    k_mean::execute();
                }
                2 => {
                    classification::execute();
                }
                3 => {
                    two_classes_function::execute();
                }
                4 => {
                    n_classes_functions::execute();
                }
                5 => {
                    hierarchy_grouping::execute();
                }
                _ => {
                    eprintln!("Указанного задания не существует.");
                    eprintln!("---------------------------------");
                    continue;
                }
            }
        } else {
            eprintln!("Введено неправильное число.");
            eprintln!("---------------------------------");
            continue;
        }

        println!("---------------------------------")
    }
}
