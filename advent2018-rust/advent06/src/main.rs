use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

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

struct Coord {
    x: i32,
    y: i32
}

fn line_to_coord(line: &str) -> Option<Coord> {
    let values: Vec<i32> = line
        .splitn(2, ", ")
        .flat_map(|part| part.parse::<i32>())
        .collect();

    match values.len() {
        2 => Some(Coord {x: values[0], y: values[1]}),
        _ => None
    }
}

fn manhattan_distance(a: &Coord, b: &Coord) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

fn answer1(input: &str) -> usize {
    let coords: Vec<Coord> = input
        .split("\n")
        .flat_map(line_to_coord)
        .collect();

    // These coordinates give the extent of the grid we care about
    let xmin: i32 = coords.iter().map(|c| c.x).min().unwrap();
    let xmax: i32 = coords.iter().map(|c| c.x).max().unwrap();
    let ymin: i32 = coords.iter().map(|c| c.y).min().unwrap();
    let ymax: i32 = coords.iter().map(|c| c.y).max().unwrap();

    // If an edge coordinate is closest to coordinate X, that means
    // X's range extends to infinity -- this lets us know what to ignore
    let mut ignores: HashSet<usize> = HashSet::new();

    // We'll count the number of times each coord is the shortest distance
    // from a grid square.
    let mut counts: HashMap<usize, usize> = HashMap::new();

    for x in xmin..(xmax+1) {
        for y in ymin..(ymax+1) {
            let here = Coord {x, y};
            let mut shortest_distance: Option<usize> = None;
            let mut closest_coord_id: Option<usize> = None;

            for (coord_id, coord) in coords.iter().enumerate() {
                let distance = manhattan_distance(&coord, &here);
                match shortest_distance {
                    Some(sd) if sd < distance => {
                        // nothing to do here
                    },
                    Some(sd) if sd == distance => {
                        // it's a tie, erase the winners
                        closest_coord_id = None;
                    },
                    _ => {
                        shortest_distance = Some(distance);
                        closest_coord_id = Some(coord_id);
                    }
                }
            }

            match closest_coord_id {
                None => {},
                Some(id) => {
                    let old_count = counts.get(&id).unwrap_or(&0);
                    counts.insert(id, old_count + 1);

                    if x == xmin || x == xmax || y == ymin || y == ymax {
                        ignores.insert(id);
                    }
                }
            }
        }
    }

    for coord_id in ignores {
        counts.remove(&coord_id);
    }

    *counts.values().max().unwrap()
}

fn answer2(input: &str) -> usize {
    let coords: Vec<Coord> = input
        .split("\n")
        .flat_map(line_to_coord)
        .collect();

    let xmin: i32 = coords.iter().map(|c| c.x).min().unwrap();
    let xmax: i32 = coords.iter().map(|c| c.x).max().unwrap();
    let ymin: i32 = coords.iter().map(|c| c.y).min().unwrap();
    let ymax: i32 = coords.iter().map(|c| c.y).max().unwrap();

    let mut area = 0;

    for x in xmin..(xmax+1) {
        'next_square: for y in ymin..(ymax+1) {
            let mut total_distance = 0;
            let here = Coord {x, y};
            for (coord_id, coord) in coords.iter().enumerate() {
                let distance = manhattan_distance(&coord, &here);
                total_distance += distance;
                if total_distance >= 10000 { continue 'next_square; }
            }
            area += 1;
        }
    }

    area
}



