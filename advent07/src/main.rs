use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      let mut discs = parse_input(&input);
      let bottom_disc = find_bottom_disc(&discs).unwrap();
      println!("Part 1 answer: {}", bottom_disc);
    },
    Err(e) => println!("Error: {}", e),
  }
}

struct Disc {
  name: String,
  weight: i32,
  above_weight: i32,
  aboves: Vec<String>,
}


// The bottom disc is the one that's not a key in the aboves map
fn find_bottom_disc(discs: &Vec<Disc>) -> Option<&str> {
  let aboves = get_aboves_map(discs);
  for disc in discs {
    if None == aboves.get(&disc.name) {
      return Some(&disc.name);
    }
  }
  return None;
}

// inserting(x, y) means that x is above y
fn get_aboves_map(discs: &Vec<Disc>) -> HashMap<String, String> {
  let mut aboves = HashMap::new();
  for disc in discs {
    for above in &disc.aboves {
      aboves.insert(above.to_owned(), disc.name.to_owned());
    }
  }
  return aboves;
}

fn parse_input(input: &str) -> Vec<Disc> {
  let mut discs = input
    .lines()
    .flat_map(|line| parse_line(line))
    .collect();
  return discs;
}


fn parse_line<'a>(line: &str) -> Option<Disc> {
  let regex = Regex::new(r"(?x)
    ([a-z]+)         ## capture 1 = name
    \s+
    \((\d+)\)        ## capture 2 = weight
    (?: \s+ -> \s+)?
    ([a-z,\s]+)?     ## capture 3 = discs above
  ").unwrap();

  let caps = regex.captures(line)?;

  let name = caps.get(1)?.as_str().to_owned();
  let weight = caps.get(2)?.as_str().parse::<i32>().ok()?;
  let aboves: Vec<String>;
  let above_weight = 0;
  match caps.get(3) {
    Some(cap) => {
      aboves = cap.as_str().split(", ").map(|s| s.to_owned()).collect();
    },
    None => {
      aboves = vec![];
    },
  }

  return Some(Disc{name, weight, aboves, above_weight});
}
