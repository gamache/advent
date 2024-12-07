use std::collections::HashMap;

struct CharGrid {
    grid: HashMap<(usize, usize), char>,
    rowmax: usize,
    colmax: usize,
}

impl CharGrid {
    fn at(&self, row: usize, col: usize) -> Option<&char> {
        let modcol = col % (1 + self.colmax);
        self.grid.get(&(row, modcol))
    }
}

fn lines_to_grid(lines: Vec<String>) -> CharGrid {
    let mut cg = CharGrid {
        grid: HashMap::new(),
        rowmax: 0,
        colmax: 0,
    };
    let mut row = 0;
    let mut col = 0;

    for line in lines {
        for c in line.chars() {
            cg.grid.insert((row, col), c);
            cg.colmax = col;
            col = col + 1;
        }
        cg.rowmax = row;
        row = row + 1;
        col = 0;
    }

    cg
}

const TREE: char = '#';

pub fn run(lines: Vec<String>) -> () {
    let cg = lines_to_grid(lines);
    part1(&cg);
    part2(&cg);
}

fn count_trees(cg: &CharGrid, drow: usize, dcol: usize) -> usize {
    let mut trees = 0;
    let mut row = 0;
    let mut col = 0;

    while row <= cg.rowmax {
        if cg.at(row, col) == Some(&TREE) {
            trees = trees + 1;
        }
        row = row + drow;
        col = col + dcol;
    }

    trees
}

fn part1(cg: &CharGrid) -> () {
    println!("{}", count_trees(cg, 1, 3));
}

fn part2(cg: &CharGrid) -> () {
    println!(
        "{}",
        count_trees(cg, 1, 1)
            * count_trees(cg, 1, 3)
            * count_trees(cg, 1, 5)
            * count_trees(cg, 1, 7)
            * count_trees(cg, 2, 1)
    );
}
