use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn main() {
  match get_checksum1_from_file("input.txt") {
    Ok(checksum) => println!("Part 1 answer: {}", checksum),
    Err(e) => println!("{}", e),
  }
  match get_checksum2_from_file("input.txt") {
    Ok(checksum) => println!("Part 2 answer: {}", checksum),
    Err(e) => println!("{}", e),
  }
}

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}


fn get_line_checksum1(line: &str) -> i32 {
  let mut number_strs = line.split_whitespace();
  let mut done = false;
  let mut highest: Option<i32> = None;
  let mut lowest: Option<i32> = None;
  while !done {
    match number_strs.next() {
      None => { done = true; },
      Some(number_str) => {
        let number = match number_str.parse() { Ok(n) => { n }, _ => { 0 } };
        match highest {
          None => { highest = Some(number); },
          Some(h) => { if h < number { highest = Some(number); } },
        }
        match lowest {
          None => { lowest = Some(number); },
          Some(l) => { if l > number { lowest = Some(number); } },
        }
      },
    }
  }

  if highest == None { return 0; }
  if lowest == None { return 0; }
  return highest.unwrap() - lowest.unwrap();
}

fn get_checksum1(input: String) -> i32 {
  let mut lines = input.lines();
  let mut done = false;
  let mut checksum = 0;

  while !done {
    match lines.next() {
      None => { done  = true; },
      Some(line) => { checksum += get_line_checksum1(line); },
    }
  }

  return checksum;
}

fn get_checksum1_from_file(filename: &str) -> Result<i32, Error> {
  let input = read_input(filename)?;
  return Ok(get_checksum1(input));
}


fn get_line_checksum2(line: &str) -> i32 {
  let numbers: Vec<&str> = line.split_whitespace().collect();
  for i in 0..numbers.len() {
    for j in 0..numbers.len() {
      if i != j {
        let ni = match numbers[i].parse() { Ok(n) => n, _ => 0 };
        let nj = match numbers[j].parse() { Ok(n) => n, _ => 0 };
        if is_divisible(ni, nj) {
          println!("{} {}", ni, nj);
          return ni / nj;
        }
      }
    }
  }
  return 0;
}

fn is_divisible(x: i32, y: i32) -> bool {
  if y == 0 { return false; }
  let fx = x as f32;
  let fy = y as f32;
  let fdiv = fx / fy;
  let div = x / y;
  return fdiv == div as f32;
}

fn get_checksum2(input: String) -> i32 {
  let mut lines = input.lines();
  let mut done = false;
  let mut checksum = 0;

  while !done {
    match lines.next() {
      None => { done  = true; },
      Some(line) => { checksum += get_line_checksum2(line); },
    }
  }

  return checksum;
}

fn get_checksum2_from_file(filename: &str) -> Result<i32, Error> {
  let input = read_input(filename)?;
  return Ok(get_checksum2(input));
}

