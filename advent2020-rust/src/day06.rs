use std::collections::HashMap;

pub fn run(lines: Vec<String>) -> () {
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) -> () {
    let mut yesmap: HashMap<char, usize> = HashMap::new();
    let mut counts: usize = 0;

    for line in lines {
        if line.eq("") {
            counts = counts + yesmap.keys().count();
            yesmap = HashMap::new();
        } else {
            for c in line.chars() {
                match yesmap.get(&c) {
                    None => yesmap.insert(c, 1),
                    Some(n) => yesmap.insert(c, *n + 1),
                };
            }
        }
    }

    counts = counts + yesmap.keys().count();
    println!("{counts}")
}

fn part2(lines: &Vec<String>) -> () {
    let mut yesmap: HashMap<char, usize> = HashMap::new();
    let mut counts: usize = 0;
    let mut group_size: usize = 0;

    for line in lines {
        if line.eq("") {
            counts = counts + yesmap.values().filter(|n| **n == group_size).count();
            yesmap = HashMap::new();
            group_size = 0;
        } else {
            group_size = group_size + 1;
            for c in line.chars() {
                match yesmap.get(&c) {
                    None => yesmap.insert(c, 1),
                    Some(n) => yesmap.insert(c, *n + 1),
                };
            }
        }
    }

    counts = counts + yesmap.values().filter(|n| **n == group_size).count();
    println!("{counts}")
}
