use std::collections::HashMap;

pub fn run(lines: Vec<String>) {
    let mut numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    numbers.push(0);

    let max = numbers.iter().max().unwrap();
    numbers.push(max + 3);

    numbers.sort();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &Vec<i32>) -> () {
    let mut diff_counts: HashMap<i32, i32> = HashMap::new();

    for i in 1..numbers.len() {
        let diff: i32 = numbers[i] - numbers[i - 1];
        match diff_counts.get(&diff) {
            None => {
                diff_counts.insert(diff, 1);
            }
            Some(x) => {
                diff_counts.insert(diff, x + 1);
            }
        }
    }

    println!(
        "{}",
        diff_counts.get(&1).unwrap() * diff_counts.get(&3).unwrap()
    );
}

fn part2(numbers: &Vec<i32>) -> () {
    let mut paths: HashMap<i32, u64> = HashMap::new();
    paths.insert(0, 1);

    let nmax = numbers[numbers.len() - 1];
    for n in 1..=nmax {
        let mut npaths = 0;
        if numbers.contains(&(n - 3)) {
            npaths += paths.get(&(n - 3)).unwrap_or(&0);
        }
        if numbers.contains(&(n - 2)) {
            npaths += paths.get(&(n - 2)).unwrap_or(&0);
        }
        if numbers.contains(&(n - 1)) {
            npaths += paths.get(&(n - 1)).unwrap_or(&0);
        }
        paths.insert(n, npaths);
    }

    println!("{}", paths[&nmax]);
}
