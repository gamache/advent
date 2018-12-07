#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
//use std::iter::FromIterator;
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
            //println!("Part 2 answer: {}", answer2(&input));
        },
        Err(e) => println!("Error: {}", e),
    }
}

// boilerplate ends here

// returning Some(('X', 'Y')) means that Y depends on X
// (i.e., that X must complete before Y)
fn parse_line_to_dep(line: &str) -> Option<(char, char)> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(
            r"Step (.) must be finished before step (.) can begin"
        ).unwrap();
    }

    match LINE_RE.captures(line) {
        None => None,
        Some(caps) => {
            let mut xchars = caps[1].chars();
            let x = xchars.next().unwrap();

            let mut ychars = caps[2].chars();
            let y = ychars.next().unwrap();

            Some((x, y))
        }
    }
}

// step => [dependencies]
fn make_dep_graph(deps: &Vec<(char, char)>) -> HashMap<char, Vec<char>> {
    let mut dep_graph = HashMap::new();

    for (x, y) in deps {
        let ydeps = dep_graph.entry(*y).or_insert_with(Vec::new);
        ydeps.push(*x);

        // ensure X is present in the graph, even if we aren't adding deps to it
        if None == dep_graph.get(&x) { dep_graph.insert(*x, Vec::new()); }
    }

    dep_graph
}

fn remove_step(steps: &Vec<char>, step: char) -> Vec<char> {
    steps
        .into_iter()
        .filter(|&s| *s != step)
        .map(|&s| s)
        .collect()
}

fn answer1(input: &str) -> String {
    let deps = input
        .split("\n")
        .flat_map(parse_line_to_dep)
        .collect();

    let mut dep_graph = make_dep_graph(&deps);
    let mut steps: Vec<char> = Vec::new();

    loop {
        let mut next_step = None;
        let mut new_dep_graph = dep_graph.clone();
        let dep_keys: Vec<char> = dep_graph.keys().cloned().collect();

        // find next step
        for (step, step_deps) in dep_graph {
            if step_deps.len() > 0 { continue; }
            match next_step {
                None    => { next_step = Some(step); },
                Some(s) => { next_step = Some(std::cmp::min(step, s)); }
            }
        }

        // write it down, and remove it from dependency graph
        match next_step {
            None => {},
            Some(step) => {
                steps.push(step);
                new_dep_graph.remove(&step);

                for key in dep_keys {
                    match new_dep_graph.get(&key) {
                        Some(key_deps) => {
                            new_dep_graph.insert(key, remove_step(key_deps, step));
                        },
                        None => {}
                    }
                }
            }
        }

        dep_graph = new_dep_graph;
        if dep_graph.len() == 0 { break; }
    }

    steps.into_iter().collect()
}

