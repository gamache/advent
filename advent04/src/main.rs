use std::collections::HashMap;
use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  match read_input("input.txt") {
    Ok(input) => println!("Part 1 answer: {}", count_valid_passphrases(&input)),
    Err(e) => println!("Error: {}", e),
  }
}

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn count_valid_passphrases(input: &str) -> usize {
  return input
    .lines()
    .filter(|line| is_valid_passphrase(line))
    .count();
}

fn is_valid_passphrase(phrase: &str) -> bool {
  let mut hash: HashMap<&str, bool> = HashMap::new();
  for chunk in phrase.split_whitespace() {
    if None == hash.get(chunk) {
      hash.insert(chunk, true);
    }
    else {
      return false;
    }
  }
  return true;
}
