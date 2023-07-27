use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

//mod chargrid;
mod day15;

fn main() {
    day15::run(read_lines("inputs/day15.txt"));
}
