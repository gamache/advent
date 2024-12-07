use std::collections::HashMap;

pub fn run(lines: Vec<String>) -> () {
    run_for(&lines[0], 2020);
    run_for(&lines[0], 30000000);
}

fn run_for(line: &str, rounds: usize) -> () {
    let mut t: usize = 0;
    let mut last_spoken: HashMap<usize, usize> = HashMap::new();
    let mut last_number: usize = 22;

    line.split(",").for_each(|n| {
        t += 1;
        last_number = n.parse::<usize>().unwrap();
        last_spoken.insert(last_number, t);
    });

    while t < rounds {
        let next_number = match last_spoken.get(&last_number) {
            None => 0,
            Some(last_t) => t - last_t,
        };
        last_spoken.insert(last_number, t);

        last_number = next_number;
        t += 1;
    }

    println!("{last_number}");
}
