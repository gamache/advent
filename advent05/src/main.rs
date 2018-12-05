use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

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

fn is_upper(c: u8) -> bool { c > 96 && c < 123 }
fn is_lower(c: u8) -> bool { c > 64 && c < 91 }
fn is_letter(c: u8) -> bool { is_upper(c) || is_lower(c) }

fn is_case_variant(a: u8, b: u8) -> bool {
    is_letter(a) && is_letter(b) && (a as i32 - b as i32).abs() == 32
}

fn swap_case(c: u8) -> u8 {
    if is_lower(c) { c + 32 }
    else { c - 32 }
}

// performs a complete polymer reaction
fn react(polymer: &str) -> String {
    let reacted = react1(polymer);
    if reacted.len() == polymer.len() {
        reacted
    }
    else {
        react(reacted.as_ref())
    }
}

// performs one round of a polymer reaction
fn react1(polymer: &str) -> String {
    let mut reacted = String::new();
    let mut last_byte = None;

    for &byte in polymer.as_bytes() {
        match last_byte {
            None => {
                last_byte = Some(byte);
            },
            Some(b) => {
                if is_case_variant(byte, b) {
                    last_byte = None;
                }
                else {
                    reacted.push(b as char);
                    last_byte = Some(byte);
                }
            }
        }
    }

    match last_byte {
        Some(b) => reacted.push(b as char),
        None => {}
    }

    reacted
}

fn answer1(input: &str) -> usize {
    react(input.trim()).len()
}

fn answer2(input: &str) -> usize {
    let letters: HashSet<u8> = input.as_bytes().to_vec().drain(..).collect();

    let mut shortest_length = input.len();

    for letter in letters {
        let letter1 = format!("{}", letter as char);
        let letter2 = format!("{}", swap_case(letter) as char);
        let shorter = input
            .replace(letter1.as_ref() as &str, "")
            .replace(letter2.as_ref() as &str, "");
        let reacted = react(&shorter);
        if reacted.len() < shortest_length {
            shortest_length = reacted.len();
        }
    }

    shortest_length
}
