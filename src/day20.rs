use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Tile {
    // (0,0) is bottom left corner, (9,0) is bottom right
    grid: HashMap<(usize, usize), bool>,
    id: usize,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            grid: HashMap::new(),
            id: 0,
            xmin: 0,
            xmax: 0,
            ymin: 0,
            ymax: 0,
        }
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for yy in 0..10 {
            let y = 9 - yy;
            for x in 0..10 {
                match self.grid.get(&(x, y)) {
                    Some(true) => print!("#"),
                    _ => print!("."),
                }
            }
            println!("");
        }
    }

    fn from_str(s: &str) -> Tile {
        let tile_id_re = Regex::new(r"Tile (?<id>\d+)").unwrap();
        let mut tile = Tile::new();

        let mut line_iter = s.lines();
        match tile_id_re.captures(line_iter.next().unwrap()) {
            None => panic!("missing tile id"),
            Some(caps) => {
                tile.id = caps["id"].parse::<usize>().unwrap();
            }
        }

        let mut x = 0;
        let mut y = 10;
        loop {
            match line_iter.next() {
                None => break,
                Some(line) => {
                    x = 0;
                    y -= 1;
                    for c in line.chars() {
                        if c == '#' {
                            tile.grid.insert((x, y), true);
                        }
                        x += 1;
                    }
                }
            }
        }

        tile
    }

    fn rotate(&self) -> Tile {
        let mut tile = Tile::new();
        tile.id = self.id;

        for x in 0..10 {
            for y in 0..10 {
                let new_y = x;
                let new_x = 9 - y;
                if self.grid.get(&(x, y)) != None {
                    tile.grid.insert((new_x, new_y), true);
                }
            }
        }

        tile
    }

    fn flip(&self) -> Tile {
        let mut tile = Tile::new();
        tile.id = self.id;

        for x in 0..10 {
            for y in 0..10 {
                let new_y = 9 - x;
                let new_x = 9 - y;
                if self.grid.get(&(x, y)) != None {
                    tile.grid.insert((new_x, new_y), true);
                }
            }
        }

        tile
    }

    fn vertical_match(top_tile: &Tile, bottom_tile: &Tile) -> bool {
        for x in 0..10 {
            if top_tile.grid.get(&(x, 0)) != bottom_tile.grid.get(&(x, 9)) {
                return false;
            }
        }
        true
    }

    fn horizontal_match(left_tile: &Tile, right_tile: &Tile) -> bool {
        for y in 0..10 {
            if left_tile.grid.get(&(9, y)) != right_tile.grid.get(&(0, y)) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Space {
    tiles: HashMap<(i32, i32), Tile>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Space {
    fn new() -> Space {
        Space {
            tiles: HashMap::new(),
            xmin: 0,
            xmax: 0,
            ymin: 0,
            ymax: 0,
        }
    }

    // this returns false in the case that there are 0 vert/horiz adjacent tiles
    fn tile_fits(&self, tile: &Tile, x: i32, y: i32) -> bool {
        let mut none_adjacent = true;

        match self.tiles.get(&(x, y)) {
            Some(_) => {
                return false;
            }
            None => {}
        }

        match self.tiles.get(&(x - 1, y)) {
            Some(left_tile) => {
                none_adjacent = false;
                if !Tile::horizontal_match(left_tile, tile) {
                    return false;
                }
            }
            None => {}
        }

        match self.tiles.get(&(x + 1, y)) {
            Some(right_tile) => {
                none_adjacent = false;
                if !Tile::horizontal_match(tile, right_tile) {
                    return false;
                }
            }
            None => {}
        }

        match self.tiles.get(&(x, y + 1)) {
            Some(top_tile) => {
                none_adjacent = false;
                if !Tile::vertical_match(top_tile, tile) {
                    return false;
                }
            }
            None => {}
        }

        match self.tiles.get(&(x, y - 1)) {
            Some(bottom_tile) => {
                none_adjacent = false;
                if !Tile::vertical_match(tile, bottom_tile) {
                    return false;
                }
            }
            None => {}
        }

        none_adjacent == false
    }

    fn fit_tile(&self, tile: &Tile, x: i32, y: i32) -> Option<Tile> {
        let mut tile: Tile = tile.to_owned();

        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        tile = tile.flip();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }
        tile = tile.rotate();
        if Space::tile_fits(self, &tile, x, y) {
            return Some(tile);
        }

        None
    }
}

pub fn run(filename: &str) {
    let mut tiles: Vec<Tile> = read_to_string(filename)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|s| Tile::from_str(s))
        .collect();

    let mut space = Space::new();
    space.tiles.insert((0, 0), tiles.pop().unwrap());

    while tiles.len() > 0 {
        tiles.rotate_left(1);
        let tile = tiles[0].clone();

        'xy: for x in (space.xmin - 1)..=(space.xmax + 1) {
            for y in (space.ymin - 1)..=(space.ymax + 1) {
                match space.fit_tile(&tile, x, y) {
                    None => {}
                    Some(t) => {
                        space.tiles.insert((x, y), t);

                        if x < space.xmin {
                            space.xmin = x;
                        }
                        if x > space.xmax {
                            space.xmax = x;
                        }
                        if y < space.ymin {
                            space.ymin = y;
                        }
                        if y > space.ymax {
                            space.ymax = y;
                        }

                        tiles.remove(0);
                        break 'xy;
                    }
                }
            }
        }
    }

    let t1 = space.tiles.get(&(space.xmax, space.ymax)).unwrap();
    let t2 = space.tiles.get(&(space.xmax, space.ymin)).unwrap();
    let t3 = space.tiles.get(&(space.xmin, space.ymax)).unwrap();
    let t4 = space.tiles.get(&(space.xmin, space.ymin)).unwrap();

    let product: u64 = t1.id as u64 * t2.id as u64 * t3.id as u64 * t4.id as u64;

    println!("{product}");

    // ok part 2 go
    let mut image: HashMap<(usize, usize), bool> = HashMap::new();
    let mut imagex: usize = 0;
    let mut imagey: usize = 0;

    for spacex in space.xmin..=space.xmax {
        for spacey in space.ymin..=space.ymax {
            let tile = space.tiles.get(&(spacex, spacey)).unwrap();
            for x in 1..9 {
                for y in 1..9 {
                    match tile.grid.get(&(x, y)) {
                        Some(true) => {
                            image.insert((imagex + x - 1, imagey + y - 1), true);
                        }
                        _ => {}
                    }
                }
            }
            imagey += 8;
        }
        imagex += 8;
    }

    let xmax = (space.xmax - space.xmin) * 8 - 1;
    let ymax = (space.ymax - space.ymin) * 8 - 1;
}

const SEA_MONSTER_COORDS: Vec<(usize, usize)> = vec![
    (0, 1),
    (1, 0),
    (4, 0),
    (5, 1),
    (6, 1),
    (7, 0),
    (10, 0),
    (11, 1),
    (12, 1),
    (13, 0),
    (16, 0),
    (17, 1),
    (18, 1),
    (18, 2),
    (19, 1),
];
const SEA_MONSTER_X: usize = 20;
const SEA_MONSTER_Y: usize = 3;

fn sea_monster(image: &HashMap<(usize, usize), bool>, x: usize, y: usize) -> bool {
    SEA_MONSTER_COORDS.iter().map
}
