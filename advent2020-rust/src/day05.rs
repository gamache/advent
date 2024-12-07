pub fn run(lines: Vec<String>) -> () {
    let seat_numbers: Vec<u32> = lines.iter().map(|line| get_seat_number(&line)).collect();

    part1(&seat_numbers);
    part2(&seat_numbers);
}

fn part1(seat_numbers: &Vec<u32>) -> () {
    println!("{}", seat_numbers.iter().max().unwrap());
}

fn part2(seat_numbers: &Vec<u32>) -> () {
    for n in seat_numbers {
        if !seat_numbers.contains(&(n + 1)) && seat_numbers.contains(&(n + 2)) {
            println!("{}", *n + 1);
        }
    }
}

fn get_seat_number(seat_spec: &str) -> u32 {
    // these will be equal by the end of processing
    let mut min: u32 = 0;
    let mut max: u32 = 127;

    // these will be equal by the end of processing
    let mut seatmin: u32 = 0;
    let mut seatmax: u32 = 7;

    for c in seat_spec.chars() {
        let half = (1 + max - min) / 2;
        let seathalf = (1 + seatmax - seatmin) / 2;
        match c {
            'F' => max = max - half,
            'B' => min = min + half,
            'L' => seatmax = seatmax - seathalf,
            'R' => seatmin = seatmin + seathalf,
            _ => panic!("{}", c.to_string()),
        }
    }

    (min * 8) + seatmin
}
