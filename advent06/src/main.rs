use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
  match read_input("input.txt") {
    Ok(input) => println!("Part 1 answer: {}", part1(&input)),
    Err(e) => println!("Error: {}", e),
  }
}

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn part1(input: &str) -> i32 {
  // this needs to be at the top of this function because of the borrow
  // checker -- this means I do not understand what I am doing
  let mut banks: Vec<i32> = input
    .split_whitespace()
    .flat_map(|n| n.parse())
    .collect();

  let mut iters = 0;
  let mut done = false;
  let mut states: HashMap<String, bool> = HashMap::new();

  while !done {
    let state_str = banks_to_string(&banks);
    if None == states.get(&state_str) {
      states.insert(state_str, true);
      let largest_index = get_index_of_largest_bank(&banks);
      redistribute(&mut banks, largest_index);
      iters += 1;
    }
    else {
      done = true;
    }
  }

  return iters;
}

// returns a string representing a bank's state
fn banks_to_string(banks: &Vec<i32>) -> String {
  let bank_strings: Vec<String> = banks.iter().map(|n| n.to_string()).collect();
  return bank_strings.join(".");
}

fn get_index_of_largest_bank(banks: &Vec<i32>) -> usize {
  let mut largest: Option<i32> = None;
  let mut largest_i = 0;
  for i in 0..banks.len() {
    let val = banks[i];
    match largest {
      None => {
        largest = Some(val);
      },
      Some(cur_largest) => {
        if val > cur_largest {
          largest_i = i;
          largest = Some(val);
        }
      },
    }
  }
  return largest_i;
}

// redistributes blocks from the given index evenly across the banks
fn redistribute(banks: &mut Vec<i32>, index: usize) -> () {
  let mut blocks = banks[index];
  let mut i = index;
  banks[index] = 0;

  while blocks > 0 {
    blocks -= 1;
    i += 1;
    i %= banks.len();
    banks[i] += 1;
  }
}
