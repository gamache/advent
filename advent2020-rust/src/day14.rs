use regex::Regex;
use std::collections::HashMap;

pub fn run(lines: Vec<String>) -> () {
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) -> () {
    let mask_re = Regex::new(r"mask = (?<mask>.{36})").unwrap();
    let memset_re = Regex::new(r"mem\[(?<addr>\d+)\] = (?<value>\d+)").unwrap();

    let mut mem: HashMap<usize, usize> = HashMap::new();

    // key = position 35..0, value = 0 or 1
    let mut mask: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        match mask_re.captures(line) {
            Some(caps) => {
                mask = HashMap::new();
                let mut pos: usize = 36;
                for c in caps["mask"].chars() {
                    pos -= 1;
                    match c {
                        'X' => {}
                        '0' => {
                            mask.insert(pos, 0);
                        }
                        '1' => {
                            mask.insert(pos, 1);
                        }
                        _ => panic!("bad char in mask: {}", c),
                    }
                }
            }
            None => match memset_re.captures(line) {
                Some(caps) => {
                    let addr: usize = caps["addr"].parse::<usize>().unwrap();
                    let mut val: usize = caps["value"].parse::<usize>().unwrap();
                    for pos in mask.keys() {
                        let bit: usize = 1 << pos;
                        match mask.get(pos) {
                            Some(0) => {
                                if val & bit != 0 {
                                    val = val ^ bit;
                                }
                            }
                            Some(1) => {
                                if val & bit == 0 {
                                    val = val ^ bit;
                                }
                            }
                            _ => {}
                        }
                    }
                    mem.insert(addr, val);
                }
                None => {
                    panic!("bad match: {}", line);
                }
            },
        }
    }

    let mut sum: usize = 0;
    for val in mem.values() {
        sum += val;
    }

    println!("{sum}");
}

fn part2(lines: &Vec<String>) -> () {
    let mask_re = Regex::new(r"mask = (?<mask>.{36})").unwrap();
    let memset_re = Regex::new(r"mem\[(?<addr>\d+)\] = (?<value>\d+)").unwrap();

    let mut mem: HashMap<usize, usize> = HashMap::new();

    // key = position 35..0, value = 0 or 1
    let mut mask: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        match mask_re.captures(line) {
            Some(caps) => {
                mask = HashMap::new();
                let mut pos: usize = 36;
                for c in caps["mask"].chars() {
                    pos -= 1;
                    match c {
                        'X' => {}
                        '0' => {
                            mask.insert(pos, 0);
                        }
                        '1' => {
                            mask.insert(pos, 1);
                        }
                        _ => panic!("bad char in mask: {}", c),
                    }
                }
            }
            None => match memset_re.captures(line) {
                Some(caps) => {
                    let addr: usize = caps["addr"].parse::<usize>().unwrap();
                    let val: usize = caps["value"].parse::<usize>().unwrap();
                    let mut addrs = vec![0];
                    for invpos in 0..=35 {
                        let pos = 35 - invpos;
                        let addr_bit: usize = if addr & (1 << pos) == 0 { 0 } else { 1 };

                        match mask.get(&pos) {
                            Some(0) => {
                                for ia in 0..addrs.len() {
                                    addrs[ia] = (addrs[ia] << 1) + addr_bit;
                                }
                            }
                            Some(1) => {
                                for ia in 0..addrs.len() {
                                    addrs[ia] = (addrs[ia] << 1) + 1;
                                }
                            }
                            _ => {
                                for ia in 0..addrs.len() {
                                    addrs[ia] = addrs[ia] << 1;
                                    addrs.push(addrs[ia] + 1);
                                }
                            }
                        }
                    }

                    for a in addrs {
                        mem.insert(a, val);
                    }
                }
                None => {
                    panic!("bad match: {}", line);
                }
            },
        }
    }

    let mut sum: usize = 0;
    for val in mem.values() {
        sum += val;
    }

    println!("{sum}");
}
