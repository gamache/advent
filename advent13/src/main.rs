use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashSet;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      println!("Part 1 answer: {:?}", answer1(&input));
      println!("Part 2 answer: {:?}", answer2(&input));
    },
    Err(e) => println!("Error: {:?}", e),
  }
}

// boilerplate ends here

#[derive(PartialEq,Eq,Hash,Debug,Clone,Copy)]
struct Coord {
    x: i32,
    y: i32,
}

enum Track {
    Horizontal,
    Vertical,
    CornerSlash,
    CornerBackslash,
    Intersection,
}

#[derive(Debug,Clone)]
struct Cart {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    turns: i32,
}

impl Cart {
    // These directions are wonky because positive Y means "down" not "up"
    fn intersection_direction(&self) -> (i32, i32) {
        match self.turns % 3 {
            0 => { // left
                match (self.dx, self.dy) {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => panic!()
                }
            },
            1 => { // straight
                (self.dx, self.dy)
            },
            2 => { // right
                match (self.dx, self.dy) {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => panic!()
                }
            },
            _ => panic!()
        }
    }

    fn slash_direction(&self) -> (i32, i32) {
        match (self.dx, self.dy) {
            (1, 0) => (0, -1),
            (0, -1) => (1, 0),
            (-1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            _ => panic!()
        }
    }

    fn backslash_direction(&self) -> (i32, i32) {
        match (self.dx, self.dy) {
            (1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => panic!()
        }
    }

    fn moved_intersection(&self) -> Cart {
        let (dx, dy) = self.intersection_direction();
        Cart {
            x: self.x + dx,
            y: self.y + dy,
            dx: dx,
            dy: dy,
            turns: self.turns + 1,
        }
    }

    fn moved_slash(&self) -> Cart {
        let (dx, dy) = self.slash_direction();
        Cart {
            x: self.x + dx,
            y: self.y + dy,
            dx: dx,
            dy: dy,
            turns: self.turns,
        }
    }

    fn moved_backslash(&self) -> Cart {
        let (dx, dy) = self.backslash_direction();
        Cart {
            x: self.x + dx,
            y: self.y + dy,
            dx: dx,
            dy: dy,
            turns: self.turns,
        }
    }

    fn moved(&self) -> Cart {
        Cart {
            x: self.x + self.dx,
            y: self.y + self.dy,
            dx: self.dx,
            dy: self.dy,
            turns: self.turns,
        }
    }
}

struct Railway {
    rail_map: HashMap<Coord, Track>,
    carts: Vec<Cart>,
    crashes: Vec<Coord>,
}

impl Railway {
    fn from_input(input: &str) -> Railway {
        let mut rail_map = HashMap::new();
        let mut carts = Vec::new();
        let mut x = 0;
        let mut y = 0;

        for byte in input.as_bytes().into_iter() {
            match *byte as char {
                // whitespace
                '\n' => {
                    x = 0; y += 1;
                },
                ' ' => {
                    x += 1;
                },

                // track
                '-' => {
                    rail_map.insert(Coord {x, y}, Track::Horizontal);
                    x += 1;
                },
                '|' => {
                    rail_map.insert(Coord {x, y}, Track::Vertical);
                    x += 1;
                },
                '/' => {
                    rail_map.insert(Coord {x, y}, Track::CornerSlash);
                    x += 1;
                },
                '\\' => {
                    rail_map.insert(Coord {x, y}, Track::CornerBackslash);
                    x += 1;
                },
                '+' => {
                    rail_map.insert(Coord {x, y}, Track::Intersection);
                    x += 1;
                },

                // carts
                '>' => {
                    let cart = Cart { x, y, dx: 1, dy: 0, turns: 0 };
                    carts.push(cart);
                    rail_map.insert(Coord {x, y}, Track::Horizontal);
                    x += 1;
                },
                '<' => {
                    let cart = Cart { x, y, dx: -1, dy: 0, turns: 0 };
                    carts.push(cart);
                    rail_map.insert(Coord {x, y}, Track::Horizontal);
                    x += 1;
                },
                '^' => {
                    let cart = Cart { x, y, dx: 0, dy: -1, turns: 0 };
                    carts.push(cart);
                    rail_map.insert(Coord {x, y}, Track::Vertical);
                    x += 1;
                }
                'v' => {
                    let cart = Cart { x, y, dx: 0, dy: 1, turns: 0 };
                    carts.push(cart);
                    rail_map.insert(Coord {x, y}, Track::Vertical);
                    x += 1;
                },

                c => { panic!("bad input character {:?}", c); }
            }

        }

        Railway { rail_map, carts, crashes: Vec::new() }
    }

    fn is_cart_at_coord(&self, coord: &Coord) -> bool {
        for cart in &self.carts {
            if cart.x == coord.x && cart.y == coord.y { return true; }
        }
        false
    }

    fn tick(&mut self) -> () {
        let mut new_carts = Vec::new();
        //let mut tick_crashes = Vec::new();
        for i in 0..self.carts.len() {
            let cart = &self.carts[i];
            let coord = Coord {x: cart.x, y: cart.y};
            if self.crashes.contains(&coord) {
                continue;
            }
            let new_cart = match self.rail_map.get(&coord) {
                Some(Track::Horizontal)      => { cart.moved() },
                Some(Track::Vertical)        => { cart.moved() },
                Some(Track::CornerSlash)     => { cart.moved_slash() },
                Some(Track::CornerBackslash) => { cart.moved_backslash() },
                Some(Track::Intersection)    => { cart.moved_intersection() },
                None => { panic!("there is nothing at {:?}", coord); }
            };
            let new_coord = Coord {x: new_cart.x, y: new_cart.y};
            if self.is_cart_at_coord(&new_coord) {
                self.crashes.push(new_coord);
                //tick_crashes.push(new_coord);
                continue;
            }
            self.carts[i] = new_cart.clone();
            new_carts.push(new_cart);
        }
        self.carts = new_carts;
    }

    fn run_until_crash(&mut self) -> () {
        while self.crashes.len() == 0 { self.tick(); }
    }

    fn run_until_one_cart(&mut self) -> () {
        while self.carts.len() > 1 { self.tick(); }
    }
}

fn answer1(input: &str) -> String {
    let mut railway = Railway::from_input(input);
    railway.run_until_crash();
    let coord = &railway.crashes[0];
    format!("{},{}", coord.x, coord.y)
}

fn answer2(input: &str) -> String {
    let mut railway = Railway::from_input(input);
    railway.run_until_one_cart();
    //railway.tick();
    let cart = &railway.carts[0];
    format!("{},{}", cart.x, cart.y)
}


