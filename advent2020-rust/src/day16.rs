use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

pub fn run(lines: Vec<String>) -> () {
    let mut ranges: HashMap<String, (Range<usize>, Range<usize>)> = HashMap::new();
    let range_re: Regex =
        Regex::new(r"(?<field>.+): (?<a1>\d+)-(?<a2>\d+) or (?<b1>\d+)-(?<b2>\d+)").unwrap();

    let my_ticket: Vec<usize>;
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];

    let mut lines_iter = lines.iter();
    let mut next_line: Option<&String>;

    // parse ranges
    loop {
        next_line = lines_iter.next();
        match next_line {
            None => panic!("short read for ranges"),
            Some(line) => {
                if line == "" {
                    break;
                } else {
                    match range_re.captures(line) {
                        None => panic!(),
                        Some(caps) => {
                            let a1 = caps["a1"].parse::<usize>().unwrap();
                            let a2 = caps["a2"].parse::<usize>().unwrap();
                            let b1 = caps["b1"].parse::<usize>().unwrap();
                            let b2 = caps["b2"].parse::<usize>().unwrap();

                            ranges.insert(caps["field"].to_string(), (a1..(1 + a2), b1..(1 + b2)));
                        }
                    }
                }
            }
        }
    }

    lines_iter.next(); // toss out "your ticket:" line

    // then my ticket
    match lines_iter.next() {
        None => panic!("short read for my ticket"),
        Some(line) => {
            my_ticket = line
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
        }
    }

    lines_iter.next(); // toss out blank line
    lines_iter.next(); // toss out "nearby tickets:" line

    // then nearby tickets
    loop {
        next_line = lines_iter.next();
        match next_line {
            None => break,
            Some(line) => {
                nearby_tickets.push(
                    line.split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect(),
                );
            }
        }
    }

    // ok let's solve part 1 now

    let mut invalid_sum: usize = 0;
    let mut valid_tickets: Vec<Vec<usize>> = vec![];

    for ticket in &nearby_tickets {
        let mut ticket_is_valid = true;
        'numbers: for number in ticket {
            for (_field, range_tuple) in ranges.iter() {
                match range_tuple {
                    (r1, r2) => {
                        if r1.contains(&number) || r2.contains(&number) {
                            continue 'numbers;
                        }
                    }
                }
            }

            ticket_is_valid = false;
            invalid_sum += number;
        }
        if ticket_is_valid {
            valid_tickets.push(ticket.clone());
        }
    }

    println!("{}", invalid_sum);

    // part 2 begins here

    let mut possible_fields: HashMap<usize, Vec<String>> = HashMap::new();
    let all_fields: Vec<String> = ranges.keys().cloned().collect();

    for ticket in valid_tickets {
        let mut index: usize = 0;
        for value in &ticket {
            let mut fields: Vec<String> = match possible_fields.get(&index) {
                Some(fs) => fs.clone(),
                None => all_fields.clone(),
            };

            fields.retain(|field| {
                let (r1, r2) = &ranges.get(field).unwrap();
                r1.contains(&value) || r2.contains(&value)
            });

            possible_fields.insert(index, fields);

            index += 1;
        }
    }

    // now possible_fields contains many fields for which the Vec<String> has
    // many choices. whittle down by starting with the cases with 1 possibility

    let ticket_columns: Vec<usize> = possible_fields.keys().cloned().collect();

    loop {
        let mut more_work_to_do = false;
        for col in &ticket_columns {
            let fields = possible_fields[col].clone();
            if fields.len() == 1 {
                // remove this field from other columns' lists
                let field: &String = &fields[0];
                for col2 in &ticket_columns {
                    if col != col2 {
                        let mut new_fields = possible_fields[col2].clone();
                        new_fields.retain(|f| f != field);
                        possible_fields.insert(*col2, new_fields);
                    }
                }
            } else {
                more_work_to_do = true;
            }
        }
        if more_work_to_do == false {
            // we're done
            let mut product: u64 = 1;

            for (index, fields) in possible_fields {
                for field in fields {
                    if field.starts_with("departure") {
                        product *= my_ticket[index] as u64;
                    }
                }
            }

            println!("{product}");
            break;
        }
    }
}
