#[derive(Debug)]
struct Cups {
    numbers: Vec<usize>,
}
impl Cups {
    // prints numbers starting at 1, without the 1. ok
    fn print(&self) {
        Cups::print_numbers(&self.numbers);
    }

    fn print_numbers(numbers: &Vec<usize>) {
        let mut ns = numbers.clone();
        while ns[0] != 1 {
            ns.rotate_left(1);
        }
        for i in 1..ns.len() {
            print!("{}", ns[i]);
        }
    }

    // strategy: always keep the current number at position 0 by rotating
    // vector
    fn tick(&self) -> Cups {
        let mut numbers = self.numbers.clone();

        let cur_label = numbers[0];
        let p1 = numbers.remove(1);
        let p2 = numbers.remove(1);
        let p3 = numbers.remove(1);

        let mut dest_label = if cur_label == 1 { 9 } else { cur_label - 1 };

        let ps = vec![p1, p2, p3];
        while ps.contains(&dest_label) {
            dest_label = if dest_label == 1 { 9 } else { dest_label - 1 };
        }
        let mut dest_index: usize = 0;
        for n in &numbers {
            if *n == dest_label {
                break;
            }
            dest_index += 1;
        }
        numbers.insert(dest_index + 1, p3);
        numbers.insert(dest_index + 1, p2);
        numbers.insert(dest_index + 1, p1);

        numbers.rotate_left(1);

        Cups { numbers }
    }
}

pub fn run(numbers: Vec<usize>) {
    part1(&numbers);
    part2(&numbers);
}

pub fn part1(numbers: &Vec<usize>) {
    let mut cups = Cups {
        numbers: numbers.clone(),
    };
    for _ in 0..100 {
        cups = cups.tick();
    }
    cups.print();
}

pub fn part2(numbers: &Vec<usize>) {
    let mut nums: Vec<usize> = numbers.clone();
    for n in 10..=1_000_000 {
        nums.push(n);
    }
    let mut cups = Cups { numbers: nums };
    for i in 0..10_000_000 {
        if i % 1000 == 0 {
            println!("{i}");
        }
        cups = cups.tick();
    }
    let mut one_index: usize = 0;
    for n in &cups.numbers {
        if *n == 1 {
            break;
        }
        one_index += 1;
    }
    let x = cups.numbers[(one_index + 1) % cups.numbers.len()];
    let y = cups.numbers[(one_index + 2) % cups.numbers.len()];
    println!("{}", x * y);
}
