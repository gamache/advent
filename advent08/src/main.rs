use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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
    Ok(input) => { println!("Part 1 answer: {}", part1(&input).unwrap()); },
    Err(e) => println!("Error: {}", e),
  }
}


struct Instruction {
  reg: String,
  op: String,
  val: i32,
  cmp_reg: String,
  cmp_op: String,
  cmp_val: i32,
}

struct Machine {
  registers: HashMap<String, i32>,
  instructions: Vec<Instruction>,
  ip: usize,
  halt: bool,
}

fn part1(input: &str) -> Option<i32> {
  let mut machine = parse_input(input);
  machine.run();
  return Some(*(machine.registers.values().max()?));
}

impl Machine {
  fn run(&mut self) -> () {
    while !self.halt { self.tick(); }
  }

  fn tick(&mut self) -> () {
    if self.halt { return; }
    if self.ip >= self.instructions.len() {
      self.halt = true;
      return;
    }
    let inst = &self.instructions[self.ip];

    let cmp_reg_val;
    match self.registers.get(&inst.cmp_reg) {
      None => { cmp_reg_val = 0; },
      Some(v) => { cmp_reg_val = *v; },
    }

    let mut perform_op = false;
    match inst.cmp_op.as_ref() {
      "==" => { perform_op = cmp_reg_val == inst.cmp_val; },
      "!=" => { perform_op = cmp_reg_val != inst.cmp_val; },
      ">"  => { perform_op = cmp_reg_val > inst.cmp_val; },
      "<"  => { perform_op = cmp_reg_val < inst.cmp_val; },
      ">=" => { perform_op = cmp_reg_val >= inst.cmp_val; },
      "<=" => { perform_op = cmp_reg_val <= inst.cmp_val; },
      _ => {}
    }

    if perform_op {
      let mut reg_val = 0;
      match self.registers.get(&inst.reg) {
        Some(v) => { reg_val = *v; },
        None => {}
      }
      match inst.op.as_ref() {
        "inc" => { reg_val += inst.val; },
        "dec" => { reg_val -= inst.val; },
        _ => {}
      }
      self.registers.insert(inst.reg.clone(), reg_val);
    }

    self.ip += 1;
  }
}

fn parse_input(input: &str) -> Machine {
  let registers: HashMap<String, i32> = HashMap::new();
  let instructions = input
    .lines()
    .flat_map(|line| line_to_instruction(&line))
    .collect();
  let ip: usize = 0;
  let halt = false;
  return Machine { registers, instructions, ip, halt };
}


static VALID_OPS: [&'static str; 2] = [
  "inc",
  "dec",
];
static VALID_CMP_OPS: [&'static str; 6] = [
  "==",
  "!=",
  ">",
  "<",
  ">=",
  "<=",
];

fn line_to_instruction(line: &str) -> Option<Instruction> {
  lazy_static! {
     static ref LINE_REGEX: Regex = Regex::new(r"(?x)
      (\w+) \s+          # reg
      (inc|dec) \s+      # op
      (-?\d+) \s+        # val
      if \s+
      (\w+) \s+          # cmp_reg
      (\S+) \s+          # cmp_op
      (-?\d+)            # cmp_val
    ").unwrap();
  }

  let caps = LINE_REGEX.captures(line)?;

  let reg = caps.get(1)?.as_str().to_owned();
  let op = caps.get(2)?.as_str().to_owned();
  let val = caps.get(3)?.as_str().parse::<i32>().ok()?;
  let cmp_reg = caps.get(4)?.as_str().to_owned();
  let cmp_op = caps.get(5)?.as_str().to_owned();
  let cmp_val = caps.get(6)?.as_str().parse::<i32>().ok()?;

  if !VALID_OPS.contains(&op.as_ref()) { return None; };
  if !VALID_CMP_OPS.contains(&cmp_op.as_ref()) { return None; }

  return Some(Instruction{reg, op, val, cmp_reg, cmp_op, cmp_val});
}
