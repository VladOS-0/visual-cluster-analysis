use std::{io::stdin, process::exit, time::Instant};

use crate::{
    tasks::{
        classification, hierarchy_grouping, k_mean, maximin, n_classes_functions,
        two_classes_function,
    },
    utils::RoundToDecimalPlaces,
};

#[allow(non_upper_case_globals)]
pub mod font;
pub mod geometry;
pub mod random;
pub mod tasks;
pub mod utils;
pub mod visual;

pub fn interactive() {
    loop {
        println!("Какое задание выполнить? (1-6, 0 для выхода)");
        let mut buf = String::new();
        stdin()
            .read_line(&mut buf)
            .expect("Не удалось прочитать из стандартного ввода.");
        let index = buf.trim().parse::<usize>();
        if let Ok(index) = index {
            let start = Instant::now();
            print!("{esc}c", esc = 27 as char);
            println!("-------ЗАДАНИЕ {}-------", index);
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
                6 => {
                    maximin::execute();
                }
                _ => {
                    eprintln!("Указанного задания не существует.");
                    eprintln!("---------------------------------");
                    continue;
                }
            }

            if start.elapsed().as_millis() > 100 {
                println!(
                    "Задание {} завершено за {}s",
                    index,
                    start.elapsed().as_secs_f32().round_to_dp(2),
                );
            } else {
                println!(
                    "Задание {} завершено за {}ms",
                    index,
                    start.elapsed().as_millis(),
                );
            }
        } else {
            eprintln!("Введено неправильное число.");
            eprintln!("---------------------------------");
            continue;
        }

        println!("---------------------------------")
    }
}
