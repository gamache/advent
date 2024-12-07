use regex::Regex;

#[derive(Debug)]
struct PasswordSpec {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

pub fn run(lines: Vec<String>) -> () {
    let re = Regex::new(r"(?<min>\d+)-(?<max>\d+) (?<letter>.): (?<password>.+)\s*").unwrap();

    let password_specs: Vec<PasswordSpec> = lines
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            PasswordSpec {
                min: caps["min"].parse::<usize>().unwrap(),
                max: caps["max"].parse::<usize>().unwrap(),
                letter: caps["letter"].chars().nth(0).unwrap(),
                password: caps["password"].to_string(),
            }
        })
        .collect();

    part1(&password_specs);
    part2(&password_specs);
}

fn part1(password_specs: &Vec<PasswordSpec>) -> () {
    println!(
        "{}",
        password_specs
            .iter()
            .filter(|ps| {
                let count = ps.password.chars().filter(|c| c == &ps.letter).count();
                count >= ps.min && count <= ps.max
            })
            .count()
    );
}

fn part2(password_specs: &Vec<PasswordSpec>) -> () {
    println!(
        "{}",
        password_specs
            .iter()
            .filter(|ps| {
                let chars: Vec<char> = ps.password.chars().collect();
                let match1 = chars[ps.min - 1] == ps.letter;
                let match2 = chars[ps.max - 1] == ps.letter;
                !(match1 && match2) && (match1 || match2)
            })
            .count()
    );
}
