#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
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
            answer(&input);
        },
        Err(e) => println!("Error: {}", e),
    }
}

// boilerplate ends here

struct Point {
    x0: i64,
    y0: i64,
    dx: i64,
    dy: i64,
}

impl Point {
    fn position_at(&self, time: usize) -> Coord {
        Coord {
            x: self.x0 + self.dx * time as i64,
            y: self.y0 + self.dy * time as i64
        }
    }
}

#[derive(PartialEq,Eq,Hash)]
struct Coord {
    x: i64,
    y: i64,
}

fn line_to_point(line: &str) -> Option<Point> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(?x)
          position = < \s*
          (-?\d+) , \s*      # x0
          (-?\d+) > \s*      # y0
          velocity = < \s*
          (-?\d+) , \s*      # dx
          (-?\d+) >          # dy
        ").unwrap();
    }

    match LINE_RE.captures(line) {
        None => None,
        Some(caps) => {
            let x0 = caps[1].parse::<i64>().unwrap();
            let y0 = caps[2].parse::<i64>().unwrap();
            let dx = caps[3].parse::<i64>().unwrap();
            let dy = caps[4].parse::<i64>().unwrap();
            Some(Point { x0, y0, dx, dy })
        }
    }
}

fn answer(input: &str) -> () {
    let points: Vec<Point> = input
        .split("\n")
        .flat_map(line_to_point)
        .collect();

    let mut smallest_area = None;
    let mut smallest_time = 0;
    let mut window_xmin = None;
    let mut window_xmax = None;
    let mut window_ymin = None;
    let mut window_ymax = None;

    // find the time when the points occupy the smallest
    // total area
    for time in 0..1_000_000 {
        let mut xmin = None;
        let mut xmax = None;
        let mut ymin = None;
        let mut ymax = None;
        for point in &points {
            let coord = point.position_at(time);
            match xmax {
                Some(x) if x >= coord.x => {},
                _ => { xmax = Some(coord.x); }
            }
            match xmin {
                Some(x) if x <= coord.x => {},
                _ => { xmin = Some(coord.x); }
            }
            match ymax {
                Some(y) if y >= coord.y => {},
                _ => { ymax = Some(coord.y); }
            }
            match ymin {
                Some(y) if y <= coord.y => {},
                _ => { ymin = Some(coord.y); }
            }
        }
        let area = (xmax.unwrap() - xmin.unwrap()) * (ymax.unwrap() - ymin.unwrap());
        match smallest_area {
            Some(a) if a <= area => {},
            _ => {
                smallest_area = Some(area);
                smallest_time = time;
                window_xmin = xmin;
                window_xmax = xmax;
                window_ymin = ymin;
                window_ymax = ymax;

            }
        }
    }

    // render the sky at that time
    println!("Part 1 answer:");
    let mut sky = HashSet::new();
    for point in &points {
        let coord = point.position_at(smallest_time);
        sky.insert(coord);
    }
    for y in window_ymin.unwrap()..=window_ymax.unwrap() {
        for x in window_xmin.unwrap()..=window_xmax.unwrap() {
            match sky.contains(&Coord{x, y}) {
                false => { print!(" "); },
                true  => { print!("*"); },
            }
        }
        println!("");
    }

    println!("Part 2 answer: {}", smallest_time);

}

