use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      println!("Part 1 answer: {:?}", answer1(&input));
      println!("Part 2 answer: {:?}", answer2(&input));
    },
    Err(e) => println!("Error: {:?}", e),
  }
}

// boilerplate ends here

struct TestKitchen {
    recipe_scores: Vec<usize>,
    elf_indices: Vec<usize>,
}

impl TestKitchen {
    fn new() -> TestKitchen {
        TestKitchen {
            recipe_scores: vec![3, 7],
            elf_indices: vec![0, 1],
        }
    }

    fn tick(&mut self) -> () {
        let mut score = 0;
        for i in 0..self.elf_indices.len() {
            score += self.recipe_scores[self.elf_indices[i]];
        }

        let digits: Vec<usize> = format!("{}", score)
            .chars()
            .map(|c| (c as u8 - '0' as u8) as usize)
            .collect();
        for d in digits { self.recipe_scores.push(d); }

        for i in 0..self.elf_indices.len() {
            let recipe_score = self.recipe_scores[self.elf_indices[i]];
            self.elf_indices[i] += 1 + recipe_score;
            self.elf_indices[i] %= self.recipe_scores.len();
        }
    }

    fn last_digits(&self, n: usize) -> String {
        let len = self.recipe_scores.len();
        if len < n { return self.last_digits(len); }

        let (_, last) = self.recipe_scores.split_at(len - n);
        let last_digits: Vec<String> = last.iter().map(|i| format!("{}",i)).collect();
        last_digits.join("")
    }

    fn run_until_count(&mut self, recipe_count: usize) -> () {
        while self.recipe_scores.len() < recipe_count { self.tick(); }
    }

    // returns number of recipes to left of sequence
    fn run_until_sequence(&mut self, sequence: String) -> usize {
        let seq_len = sequence.len();
        for _ in 0..seq_len/2 { self.tick(); }

        // since we update the sequence twice during tick(), we need
        // to match twice as well
        loop {
            let last_digits = self.last_digits(seq_len + 1);

            let (last1, _) = last_digits.split_at(seq_len);
            if sequence == last1 {
                return self.recipe_scores.len() - seq_len - 1;
            }

            let (_, last2) = last_digits.split_at(1);
            if sequence == last2 {
                return self.recipe_scores.len() - seq_len;
            }

            self.tick();
        }
    }
}

fn answer1(input: &str) -> String {
    let recipe_count = input.trim().parse::<usize>().unwrap();
    let mut test_kitchen = TestKitchen::new();
    test_kitchen.run_until_count(recipe_count + 10);
    test_kitchen.last_digits(10)
}

fn answer2(input: &str) -> usize {
    let sequence = input.trim();
    let mut test_kitchen = TestKitchen::new();
    test_kitchen.run_until_sequence(sequence.to_string())
}
