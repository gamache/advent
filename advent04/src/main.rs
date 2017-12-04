use std::collections::HashMap;
use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn main() {
  match read_input("input.txt") {
    Ok(input) => println!("Part 1 answer: {}", count_valid_passphrases1(&input)),
    Err(e) => println!("Error: {}", e),
  }

  match read_input("input.txt") {
    Ok(input) => println!("Part 2 answer: {}", count_valid_passphrases2(&input)),
    Err(e) => println!("Error: {}", e),
  }
}

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn count_valid_passphrases1(input: &str) -> usize {
  return input
    .lines()
    .filter(|line| is_valid_passphrase1(line))
    .count();
}

fn is_valid_passphrase1(phrase: &str) -> bool {
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


fn count_valid_passphrases2(input: &str) -> usize {
  return input
    .lines()
    .filter(|line| is_valid_passphrase2(line))
    .count();
}

fn is_valid_passphrase2(phrase: &str) -> bool {
  let mut hash: HashMap<String, bool> = HashMap::new();
  for chunk in phrase.split_whitespace() {
    let canonical = to_canonical(chunk);
    if None == hash.get(&canonical) {
      hash.insert(canonical, true);
    }
    else {
      return false;
    }
  }
  return true;
}

// alphabetizes letters in a string
fn to_canonical(word: &str) -> String {
  let mut chars: Vec<char> = word.chars().collect();
  chars.sort();
  return String::from_iter(chars);
}

