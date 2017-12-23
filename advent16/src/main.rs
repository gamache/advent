use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashMap;
//use std::collections::HashSet;


#[macro_use] extern crate lazy_static;

extern crate regex;
use regex::Regex;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      //let (part1, part2) = answers("s1,x3/4,pe/b");
      let (part1, part2) = answers(&input);
      println!("Part 1 answer: {}", part1);
      println!("Part 2 answer: {}", part2);
    },
    Err(e) => println!("Error: {}", e),
  }
}

fn answers(input: &str) -> (String, String) {
  let mut dance = Dance {
    state: vec![
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
      'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
      //'a', 'b', 'c', 'd', 'e',
    ],
  };
  let moves = Dance::moves_from_input(input);
  let order0 = dance.order();
  dance.do_moves(&moves);
  let answer1 = dance.order();


  //for i in 1..1_000_000_000 {
    //dance.do_moves(&moves);
    //if i % 10000 == 0 { println!("{}", i); }
  //}

  let answer2 = dance.order();
  return (answer1, order0);
}


lazy_static! {
  static ref X_REGEX: Regex = Regex::new(r"(?x)
    x
    (\d+)
    /
    (\d+)
  ").unwrap();

  static ref P_REGEX: Regex = Regex::new(r"(?x)
    p
    ([a-z])
    /
    ([a-z])
  ").unwrap();

  static ref S_REGEX: Regex = Regex::new(r"(?x)
    s
    (\d+)
  ").unwrap();
}

struct DanceMove {
  move_type: char,
  s_arg: usize,
  x_arg_0: usize,
  x_arg_1: usize,
  p_arg_0: char,
  p_arg_1: char,
}

impl DanceMove {
  fn from_str(move_str: &str) -> Option<DanceMove> {
    match X_REGEX.captures(move_str) {
      Some(caps) => {
        let arg_0 = caps.get(1).unwrap().as_str().parse::<usize>().ok().unwrap();
        let arg_1 = caps.get(2).unwrap().as_str().parse::<usize>().ok().unwrap();
        return Some(DanceMove {
          move_type: 'x',
          x_arg_0: arg_0,
          x_arg_1: arg_1,

          s_arg: 0,
          p_arg_0: '\0',
          p_arg_1: '\0',
        });
      },
      None => {
        match P_REGEX.captures(move_str) {
          Some(caps) => {
            let arg_0 = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            let arg_1 = caps.get(2).unwrap().as_str().chars().nth(0).unwrap();
            return Some(DanceMove {
              move_type: 'p',
              p_arg_0: arg_0,
              p_arg_1: arg_1,

              s_arg: 0,
              x_arg_0: 0,
              x_arg_1: 0,
            });
          },
          None => {
            match S_REGEX.captures(move_str) {
              Some(caps) => {
                let arg = caps.get(1).unwrap().as_str().parse::<usize>().ok().unwrap();
                return Some(DanceMove {
                  move_type: 's',
                  s_arg: arg,

                  p_arg_0: '\0',
                  p_arg_1: '\0',
                  x_arg_0: 0,
                  x_arg_1: 0,
                });
              },
              None => { /* bad move_str */ }
            }
          }
        }
      }
    }
    return None;
  }
}

struct Dance {
  state: Vec<char>,
}

impl Dance {
  fn do_moves(&mut self, moves: &Vec<DanceMove>) -> () {
    for m in moves {
      self.do_move(m);
    }
  }

  fn moves_from_input(input: &str) -> Vec<DanceMove> {
    return input
      .split(",")
      .flat_map(|s| DanceMove::from_str(s))
      .collect();
  }

  fn order(&self) -> String {
    let mut order = String::from("");
    for i in 0..self.state.len() {
      order.push(self.state[i]);
    }
    return order;
  }

  fn do_move(&mut self, m: &DanceMove) -> () {
    if m.move_type == 'x' {
      self.move_x(m.x_arg_0, m.x_arg_1);
    }
    else if m.move_type == 'p' {
      self.move_p(m.p_arg_0, m.p_arg_1);
    }
    else if m.move_type == 's' {
      self.move_s(m.s_arg);
    }
  }

  fn index_of(&self, x: char) -> usize {
    for i in 0..self.state.len() {
      if self.state[i] == x { return i; }
    }
    return 0;
  }

  fn move_x(&mut self, a: usize, b: usize) -> () {
    let a_val = self.state[a];
    self.state[a] = self.state[b];
    self.state[b] = a_val;
  }

  fn move_p(&mut self, a: char, b: char) -> () {
    let a_i = self.index_of(a);
    let b_i = self.index_of(b);
    self.move_x(a_i, b_i);
  }

  fn move_s(&mut self, n: usize) -> () {
    if n < 1 { return; }
    let len = self.state.len();
    let last_val = self.state[len-1];
    let mut j = len-1;
    while j > 0 {
      self.state[j] = self.state[j-1];
      j -= 1;
    }
    self.state[0] = last_val;
    self.move_s(n-1);
  }
}
