use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn numeric_value(c: char) -> i32 {
  match c {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    _ => 0,
  }
}

fn compute_captcha1(input: String) -> i32 {
  // this operation is meaningless for 0-length strings
  if input.len() < 1 { return 0; }

  let mut done = false;
  let mut captcha = 0;
  let mut chars = input.chars();
  let first_char = chars.next().unwrap();
  let mut prev_char = first_char;

  while !done {
    match chars.next() {
      Some('\n') => {},
      Some(c) => {
        if prev_char == c {
          captcha += numeric_value(prev_char);
        }
        prev_char = c;
      },
      None => {
        if prev_char == first_char {
          captcha += numeric_value(prev_char);
        }
        done = true;
      }
    }
  }

  return captcha;
}

fn compute_captcha1_from_input_file(filename: &str) -> Result<i32, Error> {
  let input = read_input(filename)?;
  return Ok(compute_captcha1(input));
}


fn compute_captcha2(input: String) -> i32 {
  // this operation is meaningless for 0-length strings
  if input.len() < 1 { return 0; }

  // this operation only works on even-length strings;
  // ignore trailing char if any
  let len;
  if input.len() % 2 == 0 { len = input.len(); }
  else { len = input.len() - 1; }

  let bytes = input.as_bytes();
  let mut captcha = 0;

  for i in 0..len {
    let c = bytes[i];
    let c2 = bytes[(i + len/2) % len];
    if c == c2 {
      captcha += numeric_value(char::from(c));
    }
  }

  return captcha;
}

fn compute_captcha2_from_input_file(filename: &str) -> Result<i32, Error> {
  let input = read_input(filename)?;
  return Ok(compute_captcha2(input));
}

fn main() {
  /*
  println!("{}", compute_captcha1("1122".to_string()));
  println!("{}", compute_captcha1("1111".to_string()));
  println!("{}", compute_captcha1("1234".to_string()));
  println!("{}", compute_captcha1("91212129".to_string()));
  */

  match compute_captcha1_from_input_file("input.txt") {
    Ok(captcha) => println!("Part 1 answer: {}", captcha),
    Err(e) => println!("{}", e),
  }

  /*
  println!("{}", compute_captcha2("1212".to_string()));
  println!("{}", compute_captcha2("1221".to_string()));
  println!("{}", compute_captcha2("123425".to_string()));
  println!("{}", compute_captcha2("123123".to_string()));
  println!("{}", compute_captcha2("12131415".to_string()));
  */

  match compute_captcha2_from_input_file("input.txt") {
    Ok(captcha) => println!("Part 2 answer: {}", captcha),
    Err(e) => println!("{}", e),
  }
}

