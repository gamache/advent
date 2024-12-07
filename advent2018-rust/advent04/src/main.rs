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
      println!("Part 2 answer: {}", answer2(&input));
    },
    Err(e) => println!("Error: {}", e),
  }
}

// boilerplate ends here


// asleep_map is {guard_id => {minute => number_of_times_asleep}}
fn asleep_map(input: &str) -> HashMap<i32, HashMap<i32, i32>> {
  lazy_static! {
    static ref BEGINS_SHIFT_RE: Regex = Regex::new(r"(?x)
      \[ \d\d\d\d-\d\d-\d\d \s \d\d:(\d\d) \] \s  # minute
      Guard \s \# (\d+)                           # guard_id
    ").unwrap();

    static ref FALLS_ASLEEP_RE: Regex = Regex::new(r"(?x)
      \[ \d\d\d\d-\d\d-\d\d \s \d\d:(\d\d) \] \s  # minute
      falls \s asleep
    ").unwrap();

    static ref WAKES_UP_RE: Regex = Regex::new(r"(?x)
      \[ \d\d\d\d-\d\d-\d\d \s \d\d:(\d\d) \] \s  # minute
      wakes \s up
    ").unwrap();
  }

  let mut sorted_lines: Vec<&str> = input.trim().split("\n").collect();
  sorted_lines.sort();

  let mut asleep_map = HashMap::new();
  let mut current_guard = -1;
  let mut fell_asleep_at = -1;

  for line in sorted_lines {
    match BEGINS_SHIFT_RE.captures(line) {
      None => {},
      Some(caps) => {
        current_guard = caps[2].parse::<i32>().unwrap();
        continue;
      }
    }

    match FALLS_ASLEEP_RE.captures(line) {
      None => {},
      Some(caps) => {
        fell_asleep_at = caps[1].parse::<i32>().unwrap();
        continue;
      }
    }

    match WAKES_UP_RE.captures(line) {
      None => {},
      Some(caps) => {
        let now = caps[1].parse::<i32>().unwrap();
        let guard_asleep_map = asleep_map.entry(current_guard).or_insert_with(HashMap::new);

        for minute in fell_asleep_at..now {
          let asleep_count = *guard_asleep_map.get(&minute).unwrap_or(&0);
          guard_asleep_map.insert(minute, asleep_count + 1);
        }

        continue;
      }
    }
  }

  asleep_map
}

fn answer1(input: &str) -> i32 {
  let asleep_map = asleep_map(input);

  let mut most_sleeps = 0;
  let mut sleepiest_minute = -1;
  let mut sleepiest_guard = -1;

  for (&guard_id, guard_asleep_map) in asleep_map.iter() {
    let mut guard_sleeps = 0;
    let mut guard_most_sleeps = 0;
    let mut guard_sleepiest_minute = -1;
    for (&minute, &minute_sleeps) in guard_asleep_map.iter() {
      guard_sleeps += minute_sleeps;
      if minute_sleeps > guard_most_sleeps {
        guard_most_sleeps = minute_sleeps;
        guard_sleepiest_minute = minute;
      }
    }
    if guard_sleeps > most_sleeps {
      most_sleeps = guard_sleeps;
      sleepiest_guard = guard_id;
      sleepiest_minute = guard_sleepiest_minute;
    }
  }

  sleepiest_minute * sleepiest_guard
}

fn answer2(input: &str) -> i32 {
  let asleep_map = asleep_map(input);

  let mut most_sleeps = 0;
  let mut sleepiest_minute = -1;
  let mut sleepiest_guard = -1;

  for (&guard_id, guard_asleep_map) in asleep_map.iter() {
    for (&minute, &minute_sleeps) in guard_asleep_map.iter() {
      if minute_sleeps > most_sleeps {
        most_sleeps = minute_sleeps;
        sleepiest_minute = minute;
        sleepiest_guard = guard_id;
      }
    }
  }

  sleepiest_minute * sleepiest_guard
}
