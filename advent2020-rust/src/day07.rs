use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Containment {
    container_color: String,
    content_color: String,
    content_amount: usize,
}

pub fn run(lines: Vec<String>) -> () {
    let containments: Vec<Containment> = lines_to_containments(&lines);

    let mut containments_by_content_color: HashMap<String, Vec<Containment>> = HashMap::new();
    let mut containments_by_container_color: HashMap<String, Vec<Containment>> = HashMap::new();

    for c in containments {
        match containments_by_content_color.get_mut(&c.content_color) {
            None => {
                containments_by_content_color.insert(c.content_color.to_string(), vec![c.clone()]);
            }
            Some(cs) => {
                cs.push(c.clone());
            }
        }
        match containments_by_container_color.get_mut(&c.container_color) {
            None => {
                containments_by_container_color.insert(c.container_color.to_string(), vec![c]);
            }
            Some(cs) => {
                cs.push(c);
            }
        }
    }
    part1(&containments_by_content_color);
    part2(&containments_by_container_color);
}

fn part1(containments_by_content_color: &HashMap<String, Vec<Containment>>) -> () {
    let count = count_containing_colors(containments_by_content_color, "shiny gold".to_string());
    println!("{}", count);
}

fn part2(containments_by_container_color: &HashMap<String, Vec<Containment>>) -> () {
    let count = count_contained_colors(containments_by_container_color, "shiny gold".to_string());
    println!("{}", count);
}

fn lines_to_containments(lines: &Vec<String>) -> Vec<Containment> {
    let mut containments: Vec<Containment> = vec![];
    let line_re = Regex::new(r"(?<container_color>.+) bags contain (?<contents>.+)").unwrap();
    let content_re = Regex::new(r"(?<amount>\d+) (?<color>.+?) bags?").unwrap();

    for line in lines {
        match line_re.captures(line) {
            None => panic!("{}", line),
            Some(caps) => {
                let container_color = caps["container_color"].to_string();
                for contents_caps in content_re.captures_iter(&caps["contents"]) {
                    let content_color = contents_caps["color"].to_string();
                    let content_amount: usize = (&contents_caps["amount"]).parse().unwrap();
                    containments.push(Containment {
                        container_color: container_color.to_string(),
                        content_color,
                        content_amount,
                    });
                }
            }
        }
    }

    containments
}

fn count_containing_colors(
    containments_by_content_color: &HashMap<String, Vec<Containment>>,
    start_color: String,
) -> usize {
    let mut next_content_colors: Vec<String> = vec![start_color];
    let mut visited_colors: Vec<String> = vec![];

    while next_content_colors.len() > 0 {
        let content_color = next_content_colors.pop().unwrap();
        if !visited_colors.contains(&content_color) {
            visited_colors.push(content_color.to_string());
            match containments_by_content_color.get(&content_color) {
                None => {}
                Some(cs) => {
                    for c in cs {
                        next_content_colors.push(c.container_color.to_string());
                    }
                }
            }
        }
    }

    // println!("{:?}", containments_by_content_color);
    // println!("{:?}", visited_colors);

    // subtract 1 to account for start_color
    visited_colors.len() - 1
}

fn count_contained_colors(
    containments_by_container_color: &HashMap<String, Vec<Containment>>,
    container_color: String,
) -> usize {
    // subtract 1 to account for the outer bag
    really_count_contained_colors(containments_by_container_color, container_color) - 1
}

// if there are cycles, we're sunk, so let's assume there are none
fn really_count_contained_colors(
    containments_by_container_color: &HashMap<String, Vec<Containment>>,
    container_color: String,
) -> usize {
    match containments_by_container_color.get(&container_color) {
        None => 1,
        Some(cs) => {
            let mut count: usize = 1;
            for c in cs {
                count = count
                    + really_count_contained_colors(
                        containments_by_container_color,
                        c.content_color.to_string(),
                    ) * c.content_amount;
            }
            count
        }
    }
}
