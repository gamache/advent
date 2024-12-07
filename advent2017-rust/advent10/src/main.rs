use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      println!("Part 1 answer: {}", answer1(&input));
      //println!("Part 2 answer: {}", answer2(""));
      //println!("Part 2 answer: {}", answer2("AoC 2017"));
      //println!("Part 2 answer: {}", answer2("1,2,3"));
      //println!("Part 2 answer: {}", answer2("1,2,4"));
      println!("Part 2 answer: {}", answer2(&input));
    },
    Err(e) => println!("Error: {}", e),
  }
}

fn answer1(input: &str) -> i32 {
  let lengths: Vec<u8> = input
    .to_owned().trim()
    .split(',')
    .flat_map(|s| s.parse::<u8>())
    .collect();
  let mut knot_list = KnotList {
    list: (0..256).map(|x| x as u8).collect(),
    offset: 0,
    skip_size: 0,
  };
  knot_list.apply_lengths(&lengths);
  let v0 = knot_list.list[0];
  let v1 = knot_list.list[1];
  return (v0 as i32)*(v1 as i32);
}

fn answer2(input: &str) -> String {
  let mut lengths: Vec<u8> = input
    .to_owned().trim()
    .bytes()
    .collect();
  lengths.push(17);
  lengths.push(31);
  lengths.push(73);
  lengths.push(47);
  lengths.push(23);

  let mut knot_list = KnotList {
    list: (0..256).map(|x| x as u8).collect(),
    offset: 0,
    skip_size: 0,
  };
  for _i in 0..64 {
    knot_list.apply_lengths(&lengths);
  }
  return knot_list.get_hash_string();
}


struct KnotList {
  list: Vec<u8>,
  offset: usize,
  skip_size: usize,
}

impl KnotList {
  fn apply_lengths(&mut self, lengths: &Vec<u8>) -> () {
    for &length in lengths { self.reverse_length_and_advance(length); }
  }

  fn reverse_length_and_advance(&mut self, length: u8) -> () {
    for i in 0..(length as usize / 2) {
      let i2 = length as usize - i - 1;
      let ival = self.get(i);
      let i2val = self.get(i2);
      self.set(i, i2val);
      self.set(i2, ival);
    }

    self.offset += length as usize + self.skip_size;
    self.skip_size += 1;
  }

  fn get(&self, index: usize) -> u8 {
    let i = (self.offset + index) % self.list.len();
    return self.list[i];
  }

  fn set(&mut self, index: usize, value: u8) -> () {
    let i = (self.offset + index) % self.list.len();
    self.list[i] = value;
  }

  fn get_hash(&self) -> Vec<u8> {
    let mut hash: Vec<u8> = Vec::new();
    for i in 0..(16 as usize) {
      let mut acc: u8 = 0;
      for j in 0..(16 as usize) {
        acc ^= self.list[i*16 + j] as u8;
      }
      hash.push(acc);
    }
    return hash;
  }

  fn get_hash_string(&self) -> String {
    let hexes: Vec<String> = self
      .get_hash()
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect();
    return hexes.join("");
  }
}

