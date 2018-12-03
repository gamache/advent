#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      println!("Part 1 answer: {}", answer1(&input));
      println!("Part 2 answer: {}", answer2(&input).unwrap());
    },
    Err(e) => println!("Error: {}", e),
  }
}

// boilerplate ends here

struct Claim {
  id: i32,
  xmin: i32,
  ymin: i32,
  xmax: i32,
  ymax: i32
}

#[derive(Hash,PartialEq,Eq)]
struct Cell {
  x: i32,
  y: i32
}

fn get_cell_map(claims: &Vec<Claim>) -> HashMap<Cell,usize> {
  let mut cell_map = HashMap::new();

  for claim in claims {
    for x in claim.xmin..claim.xmax {
      for y in claim.ymin..claim.ymax {
        let cell = Cell {x, y};
        match cell_map.get(&cell) {
          None    => { cell_map.insert(cell, 1); },
          Some(1) => { cell_map.insert(cell, 2); },
          Some(_) => {} // skip the write for speed
        }
      }
    }
  }

  cell_map
}

fn answer1(input: &str) -> usize {
  let claims: Vec<Claim> = input
    .split("\n")
    .flat_map(|line| parse_line(line))
    .collect();

  get_cell_map(&claims).values().filter(|&v| *v > 1).count()
}

fn answer2(input: &str) -> Option<i32> {
  let claims: Vec<Claim> = input
    .split("\n")
    .flat_map(|line| parse_line(line))
    .collect();

  let cell_map = get_cell_map(&claims);

  'next_claim: for claim in claims {
    for x in claim.xmin..claim.xmax {
      for y in claim.ymin..claim.ymax {
        match cell_map.get(&Cell {x, y}) {
          None    => {},
          Some(1) => {},
          Some(_) => { continue 'next_claim; }
        }
      }
    }
    return Some(claim.id)
  }

  return None
}

fn parse_line(line: &str) -> Option<Claim> {
  lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"(?x)
      \#                    # literal number sign
      (\d+) \s* @ \s*       # id
      (\d+) , (\d+) : \s*   # x and y
      (\d+) x (\d+)         # width and height
    ").unwrap();
  }

  match LINE_REGEX.captures(line) {
    None => None,
    Some(caps) => match parse_caps(caps) {
      Err(_) => None,
      Ok(claim) => Some(claim)
    }
  }
}

fn parse_caps(caps: regex::Captures) -> Result<Claim, Box<std::error::Error>> {
  let id   = caps[1].parse::<i32>()?;
  let xmin = caps[2].parse::<i32>()?;
  let ymin = caps[3].parse::<i32>()?;
  let xmax = caps[4].parse::<i32>()? + xmin;
  let ymax = caps[5].parse::<i32>()? + ymin;

  Ok(Claim{id, xmin, ymin, xmax, ymax})
}

