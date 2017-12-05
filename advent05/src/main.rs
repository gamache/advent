use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  match read_input("input.txt") {
    Ok(input) => println!("Part 1 answer: {}", part1(&input).step),
    Err(e) => println!("Error: {}", e),
  }
  match read_input("input.txt") {
    Ok(input) => println!("Part 2 answer: {}", part2(&input).step),
    Err(e) => println!("Error: {}", e),
  }
}

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn part1(input: &str) -> Jumps {
  let mut jumps = Jumps::from_input(input);
  jumps.run1();
  return jumps;
}

fn part2(input: &str) -> Jumps {
  let mut jumps = Jumps::from_input(input);
  jumps.run2();
  return jumps;
}

struct Jumps {
  step: u64, // system count-up timer
  ptr: i32, // instruction pointer
  halt: bool,
  offsets: Vec<i32>,
}

impl Jumps {
  fn from_input(input: &str) -> Jumps {
    let offsets: Vec<i32> = input
      .lines()
      .flat_map(|line| line.parse())
      .collect();
    return Jumps {
      step: 0,
      ptr: 0,
      halt: false,
      offsets: offsets,
    };
  }

  fn tick1(&mut self) {
    if self.halt {
      return;
    }

    if self.ptr < 0 || self.ptr >= self.offsets.len() as i32 {
      self.halt = true;
      return;
    }

    let offset = self.offsets[self.ptr as usize];
    self.offsets[self.ptr as usize] += 1;
    self.ptr += offset;
    self.step += 1;
  }

  fn run1(&mut self) {
    while false == self.halt {
      self.tick1();
    }
  }

  fn tick2(&mut self) {
    if self.halt {
      return;
    }

    if self.ptr < 0 || self.ptr >= self.offsets.len() as i32 {
      self.halt = true;
      return;
    }

    let offset = self.offsets[self.ptr as usize];
    if offset >= 3 {
      self.offsets[self.ptr as usize] -= 1;
    }
    else {
      self.offsets[self.ptr as usize] += 1;
    }
    self.ptr += offset;
    self.step += 1;
  }

  fn run2(&mut self) {
    while false == self.halt {
      self.tick2();
    }
  }
}

