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
      let (part1, part2) = answers(&input);
      println!("Part 1 answer: {}", part1);
      println!("Part 2 answer: {}", part2);
    },
    Err(e) => println!("Error: {}", e),
  }
}

fn answers(input: &str) -> (usize, usize) {
  let mut fw1 = Firewall::from_input(input);
  fw1.run();

  let mut i = 0;
  let mut fw2 = Firewall::from_input(input);
  loop {
    fw2.reset();
    fw2.skip(i);
    fw2.run_until_caught();
    if !fw2.caught { break; }
    if i % 100 == 0 { println!("i={}", i); }
    i += 1;
  }

  return (fw1.severity, i);
}


struct Firewall {
  ranges: HashMap<usize, usize>,
  scanners: HashMap<usize, usize>,
  scanner_directions: HashMap<usize, bool>, // true == up, false == down
  packet_depth: usize,
  layers: usize,
  done: bool,
  severity: usize,
  caught: bool,
}

impl Firewall {
  fn from_input(input: &str) -> Firewall {
    let mut ranges = HashMap::new();
    let mut scanners = HashMap::new();
    let mut scanner_directions = HashMap::new();
    let packet_depth: usize = 0;
    let mut layers: usize = 0;
    let done = false;
    let severity: usize = 0;
    let caught = false;

    let pairs: Vec<Vec<usize>> = input
      .lines()
      .map(|line| line.split(": ").flat_map(|n| n.parse::<usize>()).collect())
      .collect();

    for pair in pairs {
      let depth = pair[0];
      let range = pair[1];
      ranges.insert(depth, range);
      scanners.insert(depth, 0);
      scanner_directions.insert(depth, true);
      if depth > layers { layers = depth; }
    }

    return Firewall {
      ranges,
      scanners,
      scanner_directions,
      packet_depth,
      layers,
      done,
      severity,
      caught,
    };
  }

  fn reset(&mut self) -> () {
    self.packet_depth = 0;
    self.severity = 0;
    self.caught = false;
    self.done = false;
    self.scanners = HashMap::new();
    self.scanner_directions = HashMap::new();
    for i in self.ranges.keys() {
      self.scanners.insert(*i, 0);
      self.scanner_directions.insert(*i, true);
    }
  }

  fn tick(&mut self) -> () {
    // get caught
    match self.scanners.get(&self.packet_depth) {
      None => {},
      Some(&d) => {
        if d == 0 {
          let range = self.ranges[&self.packet_depth];
          self.severity += self.packet_depth * range;
          self.caught = true;
        }
      },
    }

    // advance scanners
    for i in self.ranges.keys() {
      if self.scanner_directions[i] {
        let scanner = self.scanners[i] + 1;
        self.scanners.insert(*i, scanner);
        if scanner == self.ranges[i] - 1 {
          self.scanner_directions.insert(*i, false);
        }
      }
      else {
        let scanner = self.scanners[i] - 1;
        self.scanners.insert(*i, scanner);
        if scanner == 0 {
          self.scanner_directions.insert(*i, true);
        }
      }
    }

    // advance packet
    self.packet_depth += 1;
    if self.packet_depth > self.layers { self.done = true; }
  }

  // skips n cycles before moving packet through firewall
  fn skip(&mut self, n: usize) -> () {
    for _ in 0..n { self.tick(); }
    self.severity = 0;
    self.packet_depth = 0;
    self.done = false;
    self.caught = false;
  }

  fn run(&mut self) -> () {
    while !self.done { self.tick(); }
  }

  fn run_until_caught(&mut self) -> () {
    while !self.done && !self.caught { self.tick(); }
  }
}

