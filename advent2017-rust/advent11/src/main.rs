use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      let (part1, part2) = answers(&input);
      println!("Part 1 answer: {}", part1);
      println!("Part 2 answer: {}", part2);
    },
    Err(e) => println!("Error: {}", e),
  }
}

fn answers(input: &str) -> (i32, i32) {
  let mut hex = Hex { x: 0, y: 0, z: 0, furthest: 0 };
  let moves: Vec<&str> = input.split(",").collect();
  for direction in moves { hex.go(direction); }
  return (hex.steps_from_origin(), hex.furthest);
}


struct Hex {
  x: i32,
  y: i32,
  z: i32,
  furthest: i32,
}

impl Hex {
  fn go(&mut self, direction: &str) -> () {
    match direction {
      "n"  => { self.y += 1; self.z -= 1; },
      "ne" => { self.x += 1; self.z -= 1; },
      "se" => { self.x += 1; self.y -= 1; },
      "s"  => { self.z += 1; self.y -= 1; },
      "sw" => { self.z += 1; self.x -= 1; },
      "nw" => { self.y += 1; self.x -= 1; },
      _ => {},
    }
    let steps = self.steps_from_origin();
    if steps > self.furthest { self.furthest = steps; }
  }

  fn steps_from_origin(&self) -> i32 {
    let mut max = self.x.abs();
    let y_abs = self.y.abs();
    let z_abs = self.z.abs();
    if y_abs > max { max = y_abs; }
    if z_abs > max { max = z_abs; }
    return max;
  }
}

