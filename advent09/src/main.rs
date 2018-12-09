#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::LinkedList;
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
    scores: HashMap<usize, usize>,
    marbles: LinkedList<usize>,
    current_marble_number: usize,
}

impl GameMachine {
    fn new(player_count: usize) -> GameMachine {
        let mut marbles = LinkedList::new();
        marbles.push_front(0);
        GameMachine {
            player_count: player_count,
            scores: HashMap::new(),
            marbles: marbles,
            current_marble_number: 0,
        }
    }

    fn next_marble(&mut self) -> () {
        self.current_marble_number += 1;

        match self.current_marble_number {
            n if n % 23 == 0 => {
                let player = n % self.player_count;
                let score = n + self.remove();
                self.scores.entry(player).and_modify(|s| *s += score).or_insert(score);
            },
            n => {
                self.insert(n);
            }
        }
    }

    fn rotate_cw(&mut self) -> () {
        let marble = self.marbles.pop_front().unwrap();
        self.marbles.push_back(marble);
    }

    fn rotate_ccw(&mut self) -> () {
        let marble = self.marbles.pop_back().unwrap();
        self.marbles.push_front(marble);
    }

    fn insert(&mut self, marble: usize) -> () {
        for _ in 0..2 { self.rotate_cw(); }
        self.marbles.push_front(marble);
    }

    fn remove(&mut self) -> usize {
        for _ in 0..7 { self.rotate_ccw(); }
        self.marbles.pop_front().unwrap()
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
