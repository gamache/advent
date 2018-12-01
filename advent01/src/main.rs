use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

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

fn answer1(input: &str) -> i32 {
  let mut current_freq: i32 = 0;

  let deltas: Vec<i32> = input
    .split('\n')
    .flat_map(|s| s.parse::<i32>())
    .collect();

  deltas.iter().for_each(|d| current_freq += d);

  current_freq
}

fn answer2(input: &str) -> i32 {
  real_answer2(input, 0, HashSet::new())
}

fn real_answer2(input: &str, mut current_freq: i32, mut prev_freqs: HashSet<i32>) -> i32 {
  let deltas: Vec<i32> = input
    .split('\n')
    .flat_map(|s| s.parse::<i32>())
    .collect();

  for d in deltas {
    current_freq += d;

    if prev_freqs.contains(&current_freq) {
      return current_freq;
    }
    else {
      prev_freqs.insert(current_freq);
    }
  }

  real_answer2(input, current_freq, prev_freqs)
}

