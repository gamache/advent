pub fn run(lines: Vec<String>) -> () {
    let numbers: Vec<i64> = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    println!("{}", part1(&numbers).unwrap());
    println!("{}", part2(&numbers).unwrap());
}

fn part1(numbers: &Vec<i64>) -> Option<i64> {
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    for x in numbers {
        ix = ix + 1;
        for y in numbers {
            iy = iy + 1;
            if ix != iy && x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}

fn part2(numbers: &Vec<i64>) -> Option<i64> {
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    let mut iz: usize = 0;
    for x in numbers {
        ix = ix + 1;
        for y in numbers {
            iy = iy + 1;
            for z in numbers {
                iz = iz + 1;
                if (ix != iy || ix != iz || iy != iz) && x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}
