pub fn run(lines: Vec<String>) -> () {
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) -> () {
    let min_ts = lines[0].parse::<usize>().unwrap();
    let ids: Vec<usize> = lines[1]
        .split(",")
        .filter(|s| s != &"x")
        .map(|id| id.parse::<usize>().unwrap())
        .collect();

    let mut min_delay = min_ts;
    let mut min_id = 0;

    for id in ids {
        let delay = id - (min_ts % id);
        if delay < min_delay {
            min_delay = delay;
            min_id = id;
        }
    }

    println!("{}", min_id * min_delay)
}

fn part2(lines: &Vec<String>) -> () {
    let mut ids_and_offsets: Vec<(usize, usize)> = vec![];

    let mut i: usize = 0;
    for id in lines[1].split(",") {
        if id != "x" {
            ids_and_offsets.push((id.parse::<usize>().unwrap(), i));
        }
        i += 1;
    }

    let mut step: usize = 1;
    let mut ts: usize = 0;

    for (id, offset) in ids_and_offsets {
        while (ts + offset) % id != 0 {
            ts += step;
        }
        step *= id;
    }

    println!("{ts}");
}
