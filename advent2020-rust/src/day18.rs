use regex::Regex;

pub fn run(lines: Vec<String>) {
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut sum: u64 = 0;

    for line in lines {
        let (val, _) = compute(line.as_bytes(), 0);
        sum += val;
    }

    println!("{sum}");
}

fn part2(lines: &Vec<String>) {
    let mut sum: u64 = 0;

    for line in lines {
        let val = compute2(line);
        sum += val;
    }

    println!("{sum}");
}

// who doesn't love hand-rolling a parser?
// all numbers in input are single-digit, and everything but parens has
// spaces around it
// returns (expression value, index of last byte scanned)
fn compute(line: &[u8], start_index: usize) -> (u64, usize) {
    let mut acc: u64 = 0;
    let mut op: Option<u8> = None;
    let mut index = start_index;

    let len = line.len();

    loop {
        if index >= len {
            return (acc, len);
        }

        let b = line[index];

        if b == b'+' {
            op = Some(b'+');
        } else if b == b'*' {
            op = Some(b'*');
        } else if b >= b'0' && b <= b'9' {
            let val = (b - b'0') as u64;
            acc = match op {
                Some(b'+') => acc + val,
                Some(b'*') => acc * val,
                _ => val,
            };
            op = None;
        } else if b == b'(' {
            let (val, newindex) = compute(line, index + 1);
            index = newindex;
            acc = match op {
                Some(b'+') => acc + val,
                Some(b'*') => acc * val,
                _ => val,
            };
            op = None;
        } else if b == b')' {
            return (acc, index);
        } else if b == b' ' {
            // ignore whitespace
        } else {
            panic!("bad byte {b}");
        }

        index += 1;
    }
}

// feeling good that the function directly overhead is useless for part 2

// this pass removes parens
fn compute2(line: &str) -> u64 {
    match line.find(')') {
        None => compute2_no_parens(line),
        Some(close_paren_idx) => {
            let subline = &line[0..close_paren_idx];
            match subline.rfind('(') {
                None => panic!("close paren without open paren at {close_paren_idx}"),
                Some(open_paren_idx) => {
                    let val: u64 = compute2_no_parens(&line[(open_paren_idx + 1)..close_paren_idx]);

                    let mut s = String::new();
                    s.push_str(&line[0..open_paren_idx]);
                    s.push_str(" ");
                    s.push_str(&val.to_string());
                    s.push_str(" ");
                    s.push_str(&line[(close_paren_idx + 1)..]);

                    return compute2(&s);
                }
            };
        }
    }
}

fn compute2_no_parens(line: &str) -> u64 {
    let line = compute2_plus(line);
    compute2_times(&line)
}

fn compute2_plus(line: &str) -> String {
    let plus_re = Regex::new(r"(?<x>\d+)\s*\+\s*(?<y>\d+)").unwrap();

    let mut thisline: String = line.to_owned();

    loop {
        let newline = &plus_re
            .replace(&thisline, |caps: &regex::Captures| {
                let x = caps["x"].parse::<u64>().unwrap();
                let y = caps["y"].parse::<u64>().unwrap();
                (x + y).to_string()
            })
            .to_string();

        if newline == &thisline {
            break thisline;
        } else {
            thisline = newline.clone();
        }
    }
}

fn compute2_times(line: &str) -> u64 {
    let numbers: Vec<u64> = line
        .replace("*", " ")
        .to_string()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut product: u64 = 1;
    for n in numbers {
        product *= n;
    }
    product
}
