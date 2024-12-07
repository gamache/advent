use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
pub struct CharGrid {
    pub grid: HashMap<(usize, usize), char>,
    pub rowmax: usize,
    pub colmax: usize,
}

impl CharGrid {
    pub fn new() -> CharGrid {
        CharGrid {
            grid: HashMap::new(),
            rowmax: 0,
            colmax: 0,
        }
    }

    pub fn from_lines(lines: &Vec<String>) -> CharGrid {
        let mut cg = CharGrid::new();

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
        print!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for row in 0..=self.rowmax {
            for col in 0..=self.colmax {
                s.push(*self.get(row, col).unwrap());
            }
            s.push('\n');
        }
        s
    }
}
