use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<String, Error> {
    let mut input = String::new();
    File::open(filename)?.read_to_string(&mut input)?;
    return Ok(input);
}

fn main() {
    match read_input("input.txt") {
        Ok(input) => {
            println!("Part 1 answer: {}", answer1(&input));
            println!("Part 2 answer: {}", answer2(&input));
        },
        Err(e) => println!("Error: {}", e),
    }
}

// boilerplate ends here

fn data_from_input(input: &str) -> Vec<usize> {
    input
        .split(" ")
        .flat_map(|s| s.parse::<usize>())
        .collect()
}

struct Reader {
    metadata_total: usize,
}

impl Reader {
    fn new() -> Reader {
        Reader { metadata_total: 0 }
    }

    fn read(&mut self, data: &mut Vec<usize>) -> usize {
        let child_count = data.remove(0);
        let metadata_count = data.remove(0);

        let mut child_values: HashMap<usize, usize> = HashMap::new();
        let mut metadata: Vec<usize> = Vec::new();

        for c in 0..child_count {
            child_values.insert(c+1, self.read(data));
        }

        for _ in 0..metadata_count {
            let md = data.remove(0);
            self.metadata_total += md;
            metadata.push(md);
        }

        match child_count {
            0 => { metadata.iter().sum() },
            _ => { metadata.iter().flat_map(|i| child_values.get(&i)).sum() },
        }
    }
}

fn answer1(input: &str) -> usize {
    let mut data = data_from_input(input);
    let mut reader = Reader::new();
    reader.read(&mut data);
    reader.metadata_total
}

fn answer2(input: &str) -> usize {
    let mut data = data_from_input(input);
    let mut reader = Reader::new();
    reader.read(&mut data)
}

