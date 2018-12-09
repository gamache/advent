#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn read_input(filename: &str) -> Result<String, Error> {
    let mut input = String::new();
    File::open(filename)?.read_to_string(&mut input)?;
    return Ok(input);
}

fn main() {
    match read_input("input.txt") {
        Ok(input) => {
            println!("Part 1 answer: {}", answer1(&input));
            println!("Part 2 answer: {}", answer2(&input));
        },
        Err(e) => println!("Error: {}", e),
    }
}

// boilerplate ends here

struct GameMachine {
    player_count: usize,
    scores: HashMap<usize, usize>, // key is player number, 0-based
    marbles: Vec<usize>,
    current_marble_index: usize,
    current_marble_number: usize,
}

impl GameMachine {
    fn new(player_count: usize) -> GameMachine {
        GameMachine {
            player_count: player_count,
            scores: HashMap::new(),
            marbles: vec![0],
            current_marble_index: 0,
            current_marble_number: 0,
        }
    }

    fn next_marble(&mut self) -> () {
        self.current_marble_number += 1;

        match self.current_marble_number {
            n if n % 23 == 0 => {
                let player = n % self.player_count;
                let index = self.remove_index();
                let score = n + self.marbles.remove(index);
                self.scores.entry(player).and_modify(|s| *s += score).or_insert(score);
                self.current_marble_index = index;
            },
            n => {
                let index = self.insert_index();
                self.marbles.insert(index, n);
                self.current_marble_index = index;
            }
        }
    }

    fn insert_index(&self) -> usize {
        1 + (self.current_marble_index + 1) % self.marbles.len()
    }

    fn remove_index(&self) -> usize {
        let len = self.marbles.len();
        (self.current_marble_index + len - 7) % len
    }

    fn high_score(&self) -> usize {
        *self.scores.values().max().unwrap_or(&0)
    }

    fn run(&mut self, marble_count: usize) -> () {
        for _ in 0..marble_count { self.next_marble(); }
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(
            r"(\d+) players; last marble is worth (\d+) points"
        ).unwrap();
    }

    let caps = LINE_RE.captures(input).unwrap();
    let player_count = caps[1].parse::<usize>().unwrap();
    let marble_count = caps[2].parse::<usize>().unwrap();

    (player_count, marble_count)
}

fn answer1(input: &str) -> usize {
    let (player_count, marble_count) = parse_input(input);
    let mut game_machine = GameMachine::new(player_count);
    game_machine.run(marble_count);
    game_machine.high_score()
}

fn answer2(input: &str) -> usize {
    let (player_count, marble_count) = parse_input(input);
    let mut game_machine = GameMachine::new(player_count);
    game_machine.run(marble_count * 100);
    game_machine.high_score()
}
