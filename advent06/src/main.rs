use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      let (part1, part2) = find_loop(&input);
      println!("Part 1 answer: {}", part1);
      println!("Part 2 answer: {}", part2);
    },
    Err(e) => println!("Error: {}", e),
  }
}

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

// Returns (number of cycles before detecting loop, number of cycles in loop)
fn find_loop(input: &str) -> (i32, i32) {
  // this needs to be at the top of this function because of the borrow
  // checker -- this means I do not understand what I am doing
  let mut banks: Vec<i32> = input
    .split_whitespace()
    .flat_map(|n| n.parse())
    .collect();

  let mut loop_cycles = 0;
  let mut cycles = 0;
  let mut done = false;
  let mut states: HashMap<String, i32> = HashMap::new();

  while !done {
    let state_str = banks_to_string(&banks);
    let mut insert_state = false;
    match states.get(&state_str) {
      None => {
        insert_state = true;
        let largest_index = get_index_of_largest_bank(&banks);
        redistribute(&mut banks, largest_index);
        cycles += 1;
      },
      Some(start_cycle) => {
        done = true;
        loop_cycles = (cycles + 1) - start_cycle;
      },
    }

    // I had to break this out to outsmart the borrow checker.
    // Again, this means I do not understand what I am doing.
    if insert_state { states.insert(state_str, cycles); }
  }

  return (cycles, loop_cycles);
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
