use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::str;

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

fn answer1(input: &str) -> i32 {
  let words: Vec<&str> = input
    .trim()
    .split('\n')
    .collect();

  // key = N, where a word has exactly N of a certain letter
  // value = count of matching words
  let mut word_counts: HashMap<i32, i32> = HashMap::new();

  for word in words {
    let mut letter_counts: HashMap<u8, i32> = HashMap::new();
    let mut twos = 0;
    let mut threes = 0;

    for letter in word.as_bytes() {
      let count = match letter_counts.get(letter) {
        Some(old_count) => { old_count + 1 },
        None => 1
      };
      letter_counts.insert(*letter, count);

      if count == 2 {
        twos += 1;
      }
      else if count == 3 {
        twos -= 1;
        threes += 1;
      }
    }

    if twos > 0 {
      let twos_count = match word_counts.get(&2) {
        Some(c) => { c + 1 },
        None => 1
      };
      word_counts.insert(2, twos_count);
    }
    if threes > 0 {
      let threes_count = match word_counts.get(&3) {
        Some(c) => { c + 1 },
        None => 1
      };
      word_counts.insert(3, threes_count);
    }

  }

  let threes_count = word_counts.get(&3).unwrap_or(&0);
  let twos_count = word_counts.get(&2).unwrap_or(&0);
  threes_count * twos_count
}

fn answer2(input: &str) -> Option<String> {
  let words: Vec<&str> = input
    .trim()
    .split('\n')
    .collect();

  for ix in 0..words.len() {
    'next_word: for iy in (ix+1)..words.len() {
      if ix == iy { continue 'next_word; }

      let x = words[ix].as_bytes();
      let y = words[iy].as_bytes();

      if x.len() != y.len() { continue 'next_word; }

      let mut different = false;
      let mut index_of_different_char = 0;

      for i in 0..x.len() {
        if x[i] != y[i] {
          if different { continue 'next_word; }
          different = true;
          index_of_different_char = i;
        }
      }

      if different {
        let before = str::from_utf8(&x[0..index_of_different_char]);
        let after = str::from_utf8(&x[(index_of_different_char+1)..x.len()]);
        return Some(format!("{}{}", before.unwrap(), after.unwrap()));
      }
    }
  }

  None
}
