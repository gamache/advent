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
mod day20;

fn main() {
    day20::run("inputs/day20.txt");
    //day20::run(read_lines("inputs/day20.txt"));
}
