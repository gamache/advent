use crate::chargrid::CharGrid;

pub fn run(lines: Vec<String>) -> () {
    let cg = CharGrid::from_lines(&lines);
    part1(&cg);
    part2(&cg);
}

fn part1(cg: &CharGrid) -> () {
    let mut last_cg_str = "last".to_string();
    let mut this_cg_str = "this".to_string();
    let mut this_cg = cg.clone();

    let mut seat_count = 0;

    while last_cg_str.cmp(&this_cg_str) != std::cmp::Ordering::Equal {
        last_cg_str = this_cg.to_string();
        this_cg = tick1(&this_cg);
        this_cg_str = this_cg.to_string();
    }

    for r in 0..=this_cg.rowmax {
        for c in 0..=this_cg.colmax {
            if this_cg.get(r, c) == Some(&FILLED_SEAT) {
                seat_count += 1;
            }
        }
    }
    println!("{}", seat_count);
}

fn part2(cg: &CharGrid) -> () {
    let mut last_cg_str = "last".to_string();
    let mut this_cg_str = "this".to_string();
    let mut this_cg = cg.clone();

    let mut seat_count = 0;

    while last_cg_str.cmp(&this_cg_str) != std::cmp::Ordering::Equal {
        last_cg_str = this_cg.to_string();
        this_cg = tick2(&this_cg);
        this_cg_str = this_cg.to_string();
    }

    for r in 0..=this_cg.rowmax {
        for c in 0..=this_cg.colmax {
            if this_cg.get(r, c) == Some(&FILLED_SEAT) {
                seat_count += 1;
            }
        }
    }
    println!("{}", seat_count);
}

const EMPTY_SEAT: char = 'L';
const FILLED_SEAT: char = '#';
const FLOOR: char = '.';

fn tick1(cg: &CharGrid) -> CharGrid {
    let mut new_cg = CharGrid::new();
    for row in 0..=cg.rowmax {
        for col in 0..=cg.colmax {
            let count = occupied_adjacent1(cg, row, col);

            if count == 0 && cg.get(row, col) == Some(&EMPTY_SEAT) {
                new_cg.set(row, col, FILLED_SEAT);
            } else if count >= 4 && cg.get(row, col) == Some(&FILLED_SEAT) {
                new_cg.set(row, col, EMPTY_SEAT);
            } else {
                new_cg.set(row, col, *cg.get(row, col).unwrap())
            }
        }
    }

    // new_cg.print();
    // println!("");
    new_cg
}

fn occupied_adjacent1(cg: &CharGrid, row: usize, col: usize) -> usize {
    let mut count: usize = 0;

    for r in (row as i32 - 1)..=(row as i32 + 1) {
        for c in (col as i32 - 1)..=(col as i32 + 1) {
            if r != row as i32 || c != col as i32 {
                if cg.get(r as usize, c as usize) == Some(&FILLED_SEAT) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn tick2(cg: &CharGrid) -> CharGrid {
    let mut new_cg = CharGrid::new();
    for row in 0..=cg.rowmax {
        for col in 0..=cg.colmax {
            let count = occupied_adjacent2(cg, row, col);

            if count == 0 && cg.get(row, col) == Some(&EMPTY_SEAT) {
                new_cg.set(row, col, FILLED_SEAT);
            } else if count >= 5 && cg.get(row, col) == Some(&FILLED_SEAT) {
                new_cg.set(row, col, EMPTY_SEAT);
            } else {
                new_cg.set(row, col, *cg.get(row, col).unwrap())
            }
        }
    }
    // new_cg.print();
    // println!("");
    new_cg
}

// Returns 1 if the first seat found in the direction specified by (drow, dcol) is occupied,
// 0 otherwise.
// drow and dcol each must be -1, 0, or 1.
fn seated_in_direction(cg: &CharGrid, row: usize, col: usize, drow: i32, dcol: i32) -> usize {
    let mut r: usize = row;
    let mut c: usize = col;

    // println!("row={row} col={col}  drow={drow} dcol={dcol}:");

    loop {
        if drow > 0 && r == cg.rowmax {
            // println!("hit rowmax");
            break;
        }
        if drow < 0 && r == 0 {
            // println!("hit rowmin");
            break;
        }
        if dcol > 0 && c == cg.colmax {
            // println!("hit colmax");
            break;
        }
        if dcol < 0 && c == 0 {
            // println!("hit colmin");
            break;
        }

        r = (r as i32 + drow).try_into().unwrap();
        c = (c as i32 + dcol).try_into().unwrap();

        // print!("r={r} c={c} ");

        match cg.get(r, c) {
            Some(&EMPTY_SEAT) => {
                // println!("found empty seat, return 0");
                return 0;
            }
            Some(&FILLED_SEAT) => {
                // println!("found filled seat, return 1");
                return 1;
            }
            Some(&FLOOR) => {
                // println!("found floor, keep going");
            }
            _ => {
                panic!("wat");
            }
        }
    }

    0
}

fn occupied_adjacent2(cg: &CharGrid, row: usize, col: usize) -> usize {
    seated_in_direction(cg, row, col, 1, 0)
        + seated_in_direction(cg, row, col, 1, 1)
        + seated_in_direction(cg, row, col, 0, 1)
        + seated_in_direction(cg, row, col, -1, 1)
        + seated_in_direction(cg, row, col, -1, 0)
        + seated_in_direction(cg, row, col, -1, -1)
        + seated_in_direction(cg, row, col, 0, -1)
        + seated_in_direction(cg, row, col, 1, -1)
}
