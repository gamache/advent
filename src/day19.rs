use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Seq(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

pub fn run(lines: Vec<String>) {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut line_iter = lines.iter();

    loop {
        match line_iter.next() {
            None => panic!("short read"),
            Some(s) => {
                if s == "" {
                    break;
                }
                match s.split_once(": ") {
                    None => panic!("could not split line {s}"),
                    Some((rule_id_str, rule_spec)) => {
                        let rule_id = rule_id_str.parse::<usize>().unwrap();
                        match rule_spec.split_once(" | ") {
                            None => {
                                if rule_spec.len() == 3 && rule_spec.starts_with("\"") {
                                    rules.insert(
                                        rule_id,
                                        Rule::Literal(rule_spec.as_bytes()[1] as char),
                                    );
                                } else {
                                    let vec = rule_spec
                                        .split_ascii_whitespace()
                                        .map(|v| v.parse::<usize>().unwrap())
                                        .collect();
                                    rules.insert(rule_id, Rule::Seq(vec));
                                }
                            }
                            Some((spec1, spec2)) => {
                                let vec1 = spec1
                                    .split_ascii_whitespace()
                                    .map(|v| v.parse::<usize>().unwrap())
                                    .collect();
                                let vec2 = spec2
                                    .split_ascii_whitespace()
                                    .map(|v| v.parse::<usize>().unwrap())
                                    .collect();
                                rules.insert(rule_id, Rule::Or(vec1, vec2));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut re_str = rule_to_regex_str(&rules, 0);
    re_str.insert(0, '^');
    re_str.push('$');

    let re = Regex::new(&re_str).unwrap();

    re_str = rule_to_regex_str2(&rules, 0);
    re_str.insert(0, '^');
    re_str.push('$');

    let re2 = Regex::new(&re_str).unwrap();

    let mut matches1 = 0;
    let mut matches2 = 0;
    loop {
        match line_iter.next() {
            None => break,
            Some(s) => {
                if re.is_match(&s) {
                    matches1 += 1;
                }
                if re2.is_match(&s) {
                    matches2 += 1;
                }
            }
        }
    }
    println!("{matches1}");
    println!("{matches2}");
}

fn rule_to_regex_str(rules: &HashMap<usize, Rule>, rule_id: usize) -> String {
    let mut s = String::new();
    match rules.get(&rule_id).unwrap() {
        Rule::Literal(c) => {
            s.push(*c);
        }
        Rule::Seq(seq) => {
            seq.iter()
                .map(|id| rule_to_regex_str(rules, *id))
                .for_each(|re| s.push_str(&re));
        }
        Rule::Or(seq1, seq2) => {
            s.push_str("(");

            seq1.iter()
                .map(|id| rule_to_regex_str(rules, *id))
                .for_each(|re| s.push_str(&re));

            s.push_str("|");

            seq2.iter()
                .map(|id| rule_to_regex_str(rules, *id))
                .for_each(|re| s.push_str(&re));

            s.push_str(")");
        }
    }
    s
}

fn rule_to_regex_str2(rules: &HashMap<usize, Rule>, rule_id: usize) -> String {
    let mut s = String::new();

    if rule_id == 8 {
        s.push_str("(");
        s.push_str(&rule_to_regex_str2(rules, 42));
        s.push_str(")+");
    } else if rule_id == 11 {
        let rule42 = rule_to_regex_str(rules, 42);
        let rule31 = rule_to_regex_str(rules, 31);

        // HACK only matches 1-5 of each
        s.push_str("(");

        s.push_str(&rule42);
        s.push_str(&rule31);

        s.push_str("|");

        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule31);
        s.push_str(&rule31);

        s.push_str("|");

        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule31);
        s.push_str(&rule31);
        s.push_str(&rule31);

        s.push_str("|");

        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule31);
        s.push_str(&rule31);
        s.push_str(&rule31);
        s.push_str(&rule31);

        s.push_str("|");

        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule42);
        s.push_str(&rule31);
        s.push_str(&rule31);
        s.push_str(&rule31);
        s.push_str(&rule31);
        s.push_str(&rule31);

        s.push_str(")");
    } else {
        match rules.get(&rule_id).unwrap() {
            Rule::Literal(c) => {
                s.push(*c);
            }
            Rule::Seq(seq) => {
                seq.iter()
                    .map(|id| rule_to_regex_str2(rules, *id))
                    .for_each(|re| s.push_str(&re));
            }
            Rule::Or(seq1, seq2) => {
                s.push_str("(");

                seq1.iter()
                    .map(|id| rule_to_regex_str2(rules, *id))
                    .for_each(|re| s.push_str(&re));

                s.push_str("|");

                seq2.iter()
                    .map(|id| rule_to_regex_str2(rules, *id))
                    .for_each(|re| s.push_str(&re));

                s.push_str(")");
            }
        }
    }

    s
}
