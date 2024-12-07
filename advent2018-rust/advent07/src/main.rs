#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
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

// dep_graph is like {step => [dependencies]}
fn dep_graph_from_input(input: &str) -> HashMap<char, Vec<char>> {
    let deps: Vec<(char, char)> = input
        .split("\n")
        .flat_map(parse_line_to_dep)
        .collect();

    let mut dep_graph = HashMap::new();

    for (x, y) in deps {
        let ydeps = dep_graph.entry(y).or_insert_with(Vec::new);
        ydeps.push(x);

        // ensure X is present in the graph, even if we aren't adding deps to it
        if None == dep_graph.get(&x) { dep_graph.insert(x, Vec::new()); }
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

fn steps_from_dep_graph(graph: &HashMap<char, Vec<char>>) -> Vec<char> {
    let mut steps: Vec<char> = Vec::new();
    let mut dep_graph = graph.clone();

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

    steps
}

fn answer1(input: &str) -> String {
    let dep_graph = dep_graph_from_input(&input);
    let steps = steps_from_dep_graph(&mut dep_graph.clone());
    steps.into_iter().collect()
}

fn answer2(input: &str) -> usize {
    let mut step_machine = StepMachine::from_input(input);
    step_machine.run();
    step_machine.time
}

const N_WORKERS: usize = 5;

struct StepMachine {
    time: usize,
    dep_graph: HashMap<char, Vec<char>>,
    steps_left: Vec<char>,
    steps_done: HashSet<char>,
    working_on: HashMap<usize, char>,
    until: HashMap<usize, usize>,
    nworkers: usize,
    finished: bool,
}

impl StepMachine {
    fn step_time(step: char) -> usize {
        61 + (step as usize - 'A' as usize)
    }

    fn from_input(input: &str) -> StepMachine {
        let dep_graph = dep_graph_from_input(input);
        let steps = steps_from_dep_graph(&dep_graph);
        StepMachine {
            time: 0,
            dep_graph: dep_graph,
            steps_left: steps,
            steps_done: HashSet::new(),
            working_on: HashMap::new(),
            until: HashMap::new(),
            nworkers: N_WORKERS,
            finished: false,
        }
    }

    fn run(&mut self) -> () {
        while !self.finished { self.tick(); }
    }

    fn tick(&mut self) -> () {
        // clear out steps that just finished

        for worker in 0..self.nworkers {
            match self.until.get(&worker) {
                Some(&t) if t == self.time => {
                    let step = *self.working_on.get(&worker).unwrap();
                    self.steps_done.insert(step);
                    self.working_on.remove(&worker);
                    self.until.remove(&worker);
                },
                Some(&t) if t < self.time => {
                    let step = *self.working_on.get(&worker).unwrap();
                    panic!("step {} done at {} instead of {}", step, self.time, t);
                },
                _ => {}
            }
        }

        // assign new work

        let eligible_steps: Vec<char> = self.steps_left
            .clone()
            .into_iter()
            .filter(|&step| !self.is_blocked(step))
            .collect();

        'next_step: for step in eligible_steps {
            for worker in 0..self.nworkers {
                match self.working_on.get(&worker) {
                    None => {
                        self.steps_left = self.steps_left
                            .clone()
                            .into_iter()
                            .filter(|&s| s != step)
                            .collect();

                        self.working_on.insert(worker, step);
                        self.until.insert(worker, self.time + StepMachine::step_time(step));

                        continue 'next_step;
                    },
                    _ => {}
                }
            }
        }

        // stop or don't

        if self.working_on.len() == 0 && self.steps_left.len() == 0 {
            self.finished = true;
            return;
        }

        self.time += 1;
    }

    // returns false if step's dependencies are met, true otherwise
    fn is_blocked(&self, step: char) -> bool {
        let deps = self.dep_graph.get(&step).unwrap();
        for dep in deps {
            if !self.steps_done.contains(&dep) { return true; }
        }
        false
    }
}
