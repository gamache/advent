use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum Dirs {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

// "odd-r" horizontal layout, from https://www.redblobgames.com/grids/hexagons/
fn move_in_direction(coord: (i32, i32), dir: &Dirs) -> (i32, i32) {
    let (x, y) = coord;
    let y_is_odd = if y % 2 == 0 { 0 } else { 1 };
    let y_is_even = if y % 2 == 0 { 1 } else { 0 };
    match dir {
        Dirs::E => (x + 1, y),
        Dirs::W => (x - 1, y),
        Dirs::NE => (x + y_is_odd, y - 1),
        Dirs::NW => (x - y_is_even, y - 1),
        Dirs::SE => (x + y_is_odd, y + 1),
        Dirs::SW => (x - y_is_even, y + 1),
    }
}

fn move_in_dirs(coord: (i32, i32), dirs: &Vec<Dirs>) -> (i32, i32) {
    let mut c = coord;
    for dir in dirs {
        // print!("{:?} moves {:?} to ", c, dir);
        c = move_in_direction(c, dir);
        // println!("{:?}", c);
    }
    c
}

pub fn run(lines: Vec<String>) {
    let dir_re = Regex::new(r"(?<dir>ne|nw|se|sw|e|w)").unwrap();
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let dirs: Vec<Dirs> = dir_re
            .captures_iter(&line)
            .map(|caps| match &caps["dir"] {
                "e" => Dirs::E,
                "w" => Dirs::W,
                "ne" => Dirs::NE,
                "nw" => Dirs::NW,
                "se" => Dirs::SE,
                "sw" => Dirs::SW,
                _ => panic!("bad direction"),
            })
            .collect();

        let coord = move_in_dirs((0, 0), &dirs);

        if black_tiles.contains(&coord) {
            black_tiles.remove(&coord);
        } else {
            black_tiles.insert(coord);
        }
    }

    println!("{}", black_tiles.len());

    // part 2

    for i in 0..100 {
        let mut xmin = 10000;
        let mut xmax = -10000;
        let mut ymin = 10000;
        let mut ymax = -10000;

        for (x, y) in &black_tiles {
            if xmax < *x {
                xmax = *x;
            }
            if xmin > *x {
                xmin = *x;
            }
            if ymax < *y {
                ymax = *y;
            }
            if ymin > *y {
                ymin = *y;
            }
        }

        let mut new_black_tiles = HashSet::new();

        for x in (xmin - 2)..=(xmax + 2) {
            for y in (ymin - 2)..=(ymax + 2) {
                let mut adjs = vec![
                    move_in_direction((x, y), &Dirs::E),
                    move_in_direction((x, y), &Dirs::W),
                    move_in_direction((x, y), &Dirs::NE),
                    move_in_direction((x, y), &Dirs::NW),
                    move_in_direction((x, y), &Dirs::SE),
                    move_in_direction((x, y), &Dirs::SW),
                ];
                adjs.retain(|coord| black_tiles.contains(&coord));
                let black_count = adjs.len();
                if black_tiles.contains(&(x, y)) {
                    // this is a black tile
                    if black_count == 0 || black_count > 2 {
                        // flip to white
                    } else {
                        // remain black
                        new_black_tiles.insert((x, y));
                    }
                } else {
                    // this is a white tile
                    if black_count == 2 {
                        // flip to black
                        new_black_tiles.insert((x, y));
                    } else {
                        // remain white
                    }
                }
            }
        }

        black_tiles = new_black_tiles;
    }

    println!("{}", black_tiles.len());
}
