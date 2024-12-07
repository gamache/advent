pub fn run(lines: Vec<String>) -> () {
    let numbers: Vec<i64> = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &Vec<i64>) -> () {
    println!("{}", first_invalid(numbers));
}

fn part2(numbers: &Vec<i64>) -> () {
    println!("{}", encryption_weakness(numbers));
}

fn first_invalid(numbers: &Vec<i64>) -> i64 {
    for i in 25..numbers.len() {
        if !is_valid(numbers, i, 25) {
            return numbers[i];
        }
    }

    -1
}
fn is_valid(numbers: &Vec<i64>, index: usize, window_size: usize) -> bool {
    for j in (index - window_size)..index {
        for k in (j + 1)..index {
            let nj = numbers[j];
            let nk = numbers[k];
            if nj != nk && numbers[index] == nj + nk {
                return true;
            }
        }
    }
    false
}

fn encryption_weakness(numbers: &Vec<i64>) -> i64 {
    let target = first_invalid(numbers);
    for j in 0..numbers.len() {
        for k in (j + 1)..numbers.len() {
            let slice = &numbers[j..=k];
            let sum: i64 = slice.iter().sum();
            if sum == target {
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            }
        }
    }

    -1
}
