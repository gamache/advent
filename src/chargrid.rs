use std::collections::HashMap;

pub struct CharGrid {
    grid: HashMap<(usize, usize), char>,
    rowmax: usize,
    colmax: usize,
}

impl CharGrid {
    pub fn from_lines(lines: &Vec<String>) -> CharGrid {
        let mut cg = CharGrid {
            grid: HashMap::new(),
            rowmax: 0,
            colmax: 0,
        };

        let mut row = 0;
        let mut col = 0;

        for line in lines {
            for c in line.chars() {
                cg.set(row, col, c);
                col += 1;
            }
            row += 1;
            col = 0;
        }

        cg
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.grid.get(&(row, col))
    }

    pub fn set(&mut self, row: usize, col: usize, val: char) -> () {
        self.grid.insert((row, col), val);
        self.rowmax = row.max(self.rowmax);
        self.colmax = col.max(self.colmax);
    }

    pub fn print(&self) -> () {
        for row in 0..=self.rowmax {
            for col in 0..=self.colmax {
                print!("{}", self.get(row, col).unwrap());
            }
            println!("");
        }
    }
}
