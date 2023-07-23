use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

mod day09;

fn main() {
    day09::run(read_lines("inputs/day09.txt"));
}
