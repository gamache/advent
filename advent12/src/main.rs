use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use] extern crate lazy_static;

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
      let (part1, part2) = answers(&input);
      println!("Part 1 answer: {}", part1);
      println!("Part 2 answer: {}", part2);
    },
    Err(e) => println!("Error: {}", e),
  }
}

fn answers(input: &str) -> (usize, i32) {
  let connections_map = parse_input(input);
  let part1 = get_programs_connected_to(connections_map, 0).len();
  return (part1, 0);
}

fn get_programs_connected_to(map: HashMap<i32, Vec<i32>>, id: i32) -> Vec<i32> {
  let mut programs: HashSet<i32> = HashSet::new();
  programs.insert(id);

  let mut programs_to_check: Vec<i32> = vec![id];

  while let Some(from_prog) = programs_to_check.pop() {
    match map.get(&from_prog) {
      None => {},
      Some(to_progs) => {
        for to_prog in to_progs {
          if !programs.contains(to_prog) {
            programs.insert(*to_prog);
            programs_to_check.push(*to_prog);
          }
        }
      }
    }
  }

  return programs.iter().cloned().collect();
}

fn parse_input(input: &str) -> HashMap<i32, Vec<i32>> {
  let mut connections = HashMap::new();

  let line_conns: Vec<Connection> = input
    .lines()
    .flat_map(|line| Connection::from_line(line))
    .collect();
  for conn in line_conns { connections.insert(conn.from, conn.to); }

  return connections;
}

struct Connection {
  from: i32,
  to: Vec<i32>,
}

impl Connection {
  fn from_line(line: &str) -> Option<Connection> {
    lazy_static! {
      static ref LINE_REGEX: Regex = Regex::new(r"(?x)
        (\d+) \s+   ## from
        <-> \s+
        ([\d,\s]+)  ## to
      ").unwrap();
    }

    let caps = LINE_REGEX.captures(line)?;

    let from = caps.get(1)?.as_str().parse::<i32>().ok()?;
    let to = caps.get(2)?.as_str()
      .split(", ")
      .flat_map(|n| n.parse::<i32>())
      .collect();
    return Some(Connection { from, to });
  }
}



