#[macro_use]
extern crate lazy_static;

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

mod day04;

fn main() {
    day04::run(read_lines("inputs/day04.txt"));
}
