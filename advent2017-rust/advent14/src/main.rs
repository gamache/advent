use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
  let input = "amgozmfv";
  //let input = "flqrgnkx";
  println!("Part 1 answer amgozmfv: {}", count_all_bits(input));
  println!("Part 2 answer: {}", 0);
}

fn count_all_bits(input: &str) -> usize {
  let mut count = 0;
  for n in 0..128 {
    count += count_bits(get_knot_str(input, n).as_ref());
    //println!("{} {} {}", string, knot_str, count);
  }
  return count;
}

fn count_regions(hex_str: &str) -> usize {
  let mut region: [[usize; 128]; 128];
  for n in 0..128 {
    let knot_str = get_knot_str(hex_str, n);
  }
  return 0;
}

fn get_knot_str(input: &str, n: usize) -> String {
  let mut string = String::from(input);
  string.push('-');
  string.push_str(n.to_string().as_ref());
  return knot_hash(string.as_ref());
}

fn count_bits(hex_str: &str) -> usize {
  return hex_str
    .chars()
    .map(|b| hex_char_to_usize(b))
    .map(|b| nbits[b])
    .sum();
}

fn hex_char_to_usize(c: char) -> usize {
  let b = c as usize;
  if b >= 0x30 && b <= 0x39 { return b & 0xF; }
  else if b >= 0x61 && b <= 0x66 { return 9 + (b & 0xF); }
  return 0;
}

static nbits: [usize; 16] = [
  0, // 0000
  1, // 0001
  1, // 0010
  2, // 0011
  1, // 0100
  2, // 0101
  2, // 0110
  3, // 0111
  1, // 1000
  2, // 1001
  2, // 1010
  3, // 1011
  2, // 1100
  3, // 1101
  3, // 1110
  4  // 1111
];



// lots of copypasta from day 10

fn knot_hash(input: &str) -> String {
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

