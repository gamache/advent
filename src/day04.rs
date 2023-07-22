use regex::Regex;
use std::cmp::Ordering::*;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    cid: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            cid: None,
            pid: None,
        }
    }
}

fn lines_to_passports(lines: Vec<String>) -> Vec<Passport> {
    let re = Regex::new(r"(?<field>\S\S\S):(?<value>\S+)").unwrap();
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Passport::new();
    for line in lines {
        if line == "" {
            passports.push(passport);
            passport = Passport::new();
        } else {
            for cap in re.captures_iter(&line) {
                match &cap["field"] {
                    "byr" => passport.byr = Some(cap["value"].to_string()),
                    "iyr" => passport.iyr = Some(cap["value"].to_string()),
                    "eyr" => passport.eyr = Some(cap["value"].to_string()),
                    "hgt" => passport.hgt = Some(cap["value"].to_string()),
                    "hcl" => passport.hcl = Some(cap["value"].to_string()),
                    "ecl" => passport.ecl = Some(cap["value"].to_string()),
                    "cid" => passport.cid = Some(cap["value"].to_string()),
                    "pid" => passport.pid = Some(cap["value"].to_string()),
                    _ => panic!(),
                }
            }
        }
    }
    passports.push(passport);

    passports
}

pub fn run(lines: Vec<String>) -> () {
    let passports = lines_to_passports(lines);
    part1(&passports);
    part2(&passports);
}

fn part1(passports: &Vec<Passport>) -> () {
    println!(
        "{}",
        passports
            .iter()
            .filter(|p| {
                match p {
                    Passport {
                        byr: Some(_),
                        iyr: Some(_),
                        eyr: Some(_),
                        hgt: Some(_),
                        hcl: Some(_),
                        ecl: Some(_),
                        cid: _,
                        pid: Some(_),
                    } => true,
                    _ => false,
                }
            })
            .count()
    );
}

fn part2(passports: &Vec<Passport>) -> () {
    println!(
        "{}",
        passports
            .iter()
            .filter(|p| {
                match p {
                    Passport {
                        byr: Some(byr),
                        iyr: Some(iyr),
                        eyr: Some(eyr),
                        hgt: Some(hgt),
                        hcl: Some(hcl),
                        ecl: Some(ecl),
                        cid: _,
                        pid: Some(pid),
                    } => {
                        byr_is_valid(&byr)
                            && iyr_is_valid(&iyr)
                            && eyr_is_valid(&eyr)
                            && hgt_is_valid(&hgt)
                            && hcl_is_valid(&hcl)
                            && ecl_is_valid(&ecl)
                            && pid_is_valid(&pid)
                    }
                    _ => false,
                }
            })
            .count()
    );
}

fn byr_is_valid(byr: &str) -> bool {
    byr.len() == 4 && Less != byr.cmp("1920") && Greater != byr.cmp("2002")
}

fn iyr_is_valid(iyr: &str) -> bool {
    iyr.len() == 4 && Less != iyr.cmp("2010") && Greater != iyr.cmp("2020")
}

fn eyr_is_valid(eyr: &str) -> bool {
    eyr.len() == 4 && Less != eyr.cmp("2020") && Greater != eyr.cmp("2030")
}

lazy_static! {
    static ref HGT_RE: Regex = Regex::new(r"^(?<amount>\d+)(?<unit>cm|in)$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn hgt_is_valid(hgt: &str) -> bool {
    match HGT_RE.captures(hgt) {
        None => false,
        Some(caps) => match &caps["amount"].parse::<i32>() {
            Err(_) => false,
            Ok(amount) => match &caps["unit"] {
                "cm" => *amount >= 150 && *amount <= 193,
                "in" => *amount >= 59 && *amount <= 76,
                _ => false,
            },
        },
    }
}

fn hcl_is_valid(hcl: &str) -> bool {
    match HCL_RE.captures(hcl) {
        Some(_) => true,
        None => false,
    }
}

fn ecl_is_valid(ecl: &str) -> bool {
    match ECL_RE.captures(ecl) {
        Some(_) => true,
        None => false,
    }
}

fn pid_is_valid(pid: &str) -> bool {
    match PID_RE.captures(pid) {
        Some(_) => true,
        None => false,
    }
}
