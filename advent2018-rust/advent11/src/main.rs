use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      println!("Part 1 answer: {:?}", answer1(&input));
      println!("Part 2 answer: {:?}", answer2(&input));
    },
    Err(e) => println!("Error: {:?}", e),
  }
}

// boilerplate ends here

#[derive(PartialEq,Eq,Hash,Debug,Clone,Copy)]
struct Coord {
    x: i32,
    y: i32,
}

struct Grid {
    charge_levels: HashMap<Coord,i32>,
}

const XMAX: i32 = 300;
const YMAX: i32 = 300;

impl Grid {
    fn new(serial_number: i32) -> Grid {
        let mut grid = Grid { charge_levels: HashMap::new() };
        for x in 1..=300 {
            for y in 1..=300 {
                let coord = Coord {x, y};
                let rack_id = x + 10;
                let mut power_level = rack_id * y;
                power_level += serial_number;
                power_level *= rack_id;
                power_level = Grid::hundreds_digit(power_level) - 5;
                grid.charge_levels.insert(coord, power_level);
            }
        }
        grid
    }

    fn charge_at(&self, x: i32, y: i32) -> i32 {
        let coord = Coord {x, y};
        *self.charge_levels.get(&coord).unwrap()
    }

    fn charge_in_square(&self, coord: Coord, size: i32) -> i32 {
        let mut charge = 0;
        for x in coord.x..(coord.x + size) {
            for y in coord.y..(coord.y + size) {
                charge += self.charge_at(x, y);
            }
        }
        charge
    }

    fn hundreds_digit(n: i32) -> i32 {
        let mut digits: Vec<u8> = format!("{}", n.abs())
            .as_bytes()
            .iter()
            .map(|b| b - '0' as u8)
            .collect();
        digits.reverse();
        *digits.get(2).unwrap_or(&0) as i32
    }

    fn find_best_3x3(&self) -> Coord {
        let mut high_charge = None;
        let mut high_coord = None;
        for x in 1..=(XMAX-2) {
            for y in 1..=(YMAX-2) {
                let top_left = Coord {x, y};
                let charge = self.charge_in_square(top_left, 3);
                match high_charge {
                    Some(c) if c >= charge => {},
                    _ => {
                        high_charge = Some(charge);
                        high_coord = Some(top_left);
                    }
                }
            }
        }
        high_coord.unwrap()
    }

    fn find_best_square(&self) -> (Coord,i32) {
        let mut high_charge = None;
        let mut high_coord = None;
        let mut high_size = None;
        for x in 1..=XMAX {
            for y in 1..=YMAX {
                let top_left = Coord {x, y};
                let max_size = std::cmp::min(XMAX-x, YMAX-y);
                for size in 1..=max_size {
                    let charge = self.charge_in_square(top_left, size);
                    match high_charge {
                        Some(c) if c >= charge => {},
                        _ => {
                            high_charge = Some(charge);
                            high_coord = Some(top_left);
                            high_size = Some(size);
                        }
                    }
                }
            }
        }
        (high_coord.unwrap(), high_size.unwrap())
    }
}

// 245,14
fn answer1(input: &str) -> String {
    let serial_number = input
        .trim()
        .parse::<i32>()
        .unwrap();
    let grid = Grid::new(serial_number);
    let coord = grid.find_best_3x3();
    format!("{},{}", coord.x, coord.y)
}

// 235,206,13
fn answer2(input: &str) -> String {
    let serial_number = input
        .trim()
        .parse::<i32>()
        .unwrap();
    let grid = Grid::new(serial_number);
    let (coord, size) = grid.find_best_square();
    format!("{},{},{}", coord.x, coord.y, size)
}
