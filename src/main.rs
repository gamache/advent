use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

mod day23;

fn main() {
    day23::run(
        "158937462"
            // "389125467"
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect(),
    );
    // day24::run(read_lines("inputs/day24.txt"));
}
