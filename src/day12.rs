pub fn run(lines: Vec<String>) -> () {
    part1(&lines);
    part2(&lines);
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn left(d: Direction, deg: i32) -> Direction {
        if deg == 0 {
            return d;
        }
        let new_direction = match d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        Direction::left(new_direction, deg - 90)
    }
    fn right(d: Direction, deg: i32) -> Direction {
        if deg == 0 {
            return d;
        }
        let new_direction = match d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Direction::right(new_direction, deg - 90)
    }
    fn travel(x: i32, y: i32, direction: Direction, distance: i32) -> (i32, i32) {
        match direction {
            Direction::North => (x, y + distance),
            Direction::East => (x + distance, y),
            Direction::South => (x, y - distance),
            Direction::West => (x - distance, y),
        }
    }
}

fn part1(lines: &Vec<String>) -> () {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut direction = Direction::East;

    for line in lines {
        match (&line[0..1], line[1..].parse::<i32>().unwrap()) {
            ("N", distance) => {
                y += distance;
            }
            ("S", distance) => {
                y -= distance;
            }
            ("E", distance) => {
                x += distance;
            }
            ("W", distance) => {
                x -= distance;
            }
            ("F", distance) => match Direction::travel(x, y, direction, distance) {
                (xx, yy) => {
                    x = xx;
                    y = yy;
                }
            },
            ("L", deg) => {
                direction = Direction::left(direction, deg);
            }
            ("R", deg) => {
                direction = Direction::right(direction, deg);
            }
            _ => {
                panic!();
            }
        }
    }
    println!("{}", x.abs() + y.abs());
}

fn part2(lines: &Vec<String>) -> () {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // the waypoint's offset from the ship
    let mut wdx: i32 = 10;
    let mut wdy: i32 = 1;

    for line in lines {
        match (&line[0..1], line[1..].parse::<i32>().unwrap()) {
            ("N", distance) => {
                wdy += distance;
            }
            ("S", distance) => {
                wdy -= distance;
            }
            ("E", distance) => {
                wdx += distance;
            }
            ("W", distance) => {
                wdx -= distance;
            }
            ("F", distance) => {
                x += distance * wdx;
                y += distance * wdy;
            }
            ("L", deg) => match waypoint_left(wdx, wdy, deg) {
                (nwdx, nwdy) => {
                    wdx = nwdx;
                    wdy = nwdy;
                }
            },
            ("R", deg) => match waypoint_right(wdx, wdy, deg) {
                (nwdx, nwdy) => {
                    wdx = nwdx;
                    wdy = nwdy;
                }
            },
            _ => {
                panic!();
            }
        }
    }

    println!("{}", x.abs() + y.abs());
}

fn waypoint_left(wdx: i32, wdy: i32, deg: i32) -> (i32, i32) {
    if deg == 0 {
        return (wdx, wdy);
    }
    waypoint_left(-wdy, wdx, deg - 90)
}
fn waypoint_right(wdx: i32, wdy: i32, deg: i32) -> (i32, i32) {
    if deg == 0 {
        return (wdx, wdy);
    }
    waypoint_right(wdy, -wdx, deg - 90)
}
