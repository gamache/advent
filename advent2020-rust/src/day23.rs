use std::collections::HashMap;

pub fn run(numbers: Vec<usize>) {
    Ring::part1(&numbers);
    Ring::part2(&numbers);
}

#[derive(Clone, Debug)]
struct RingItem {
    label: usize,
    next: usize,
}

#[derive(Clone, Debug)]
struct Ring {
    items: HashMap<usize, RingItem>,
    current_label: usize,
}
impl Ring {
    fn part1_ring(input: &Vec<usize>) -> Ring {
        let mut labels_iter = input.iter();

        let first_label = *labels_iter.next().unwrap();

        let mut ring = Ring {
            items: HashMap::new(),
            current_label: first_label,
        };

        let first_item = RingItem {
            label: first_label,
            next: first_label,
        };
        ring.items.insert(first_label, first_item);

        let mut current_label = first_label;

        loop {
            match labels_iter.next() {
                None => {
                    break;
                }
                Some(label) => {
                    ring.insert(current_label, *label);
                    current_label = *label;
                }
            }
        }

        ring
    }

    fn part1(input: &Vec<usize>) {
        let mut ring = Ring::part1_ring(input);
        for _ in 0..100 {
            ring.tick();
        }
        ring.current_label = 1;
        for _ in 0..8 {
            let next_label = ring.next(ring.current_label);
            print!("{}", next_label);
            ring.current_label = next_label;
        }
        println!("");
    }

    fn part2_ring(input: &Vec<usize>) -> Ring {
        let mut labels_iter = input.iter();

        let first_label = *labels_iter.next().unwrap();

        let mut ring = Ring {
            items: HashMap::new(),
            current_label: first_label,
        };

        let first_item = RingItem {
            label: first_label,
            next: first_label,
        };
        ring.items.insert(first_label, first_item);

        let mut current_label = first_label;

        loop {
            match labels_iter.next() {
                None => {
                    break;
                }
                Some(label) => {
                    ring.insert(current_label, *label);
                    current_label = *label;
                }
            }
        }

        for i in 10..=1_000_000 {
            ring.insert(current_label, i);
            current_label = i;
        }

        ring
    }

    fn part2(input: &Vec<usize>) {
        let mut ring = Ring::part2_ring(input);
        for _ in 0..10_000_000 {
            ring.tick();
        }

        let v1 = ring.next(1);
        let v2 = ring.next(v1);
        println!("{}", v1 * v2);
    }

    fn remove(&mut self, label: usize) -> usize {
        let mut start_item = self.items.get_mut(&label).cloned().unwrap();
        let removed = self.items.get(&start_item.next).cloned().unwrap();
        start_item.next = removed.next;
        self.items.insert(start_item.label, start_item);
        self.items.remove(&removed.label);
        removed.label
    }

    fn insert(&mut self, label: usize, label_to_insert: usize) {
        let mut start_item = self.items.get(&label).cloned().unwrap();
        let insert_item = RingItem {
            label: label_to_insert,
            next: start_item.next,
        };
        start_item.next = label_to_insert;
        self.items.insert(start_item.label, start_item);
        self.items.insert(insert_item.label, insert_item);
    }

    fn next(&self, label: usize) -> usize {
        self.items.get(&label).unwrap().next
    }

    fn tick(&mut self) {
        let total_len = self.items.len();
        let r1 = self.remove(self.current_label);
        let r2 = self.remove(self.current_label);
        let r3 = self.remove(self.current_label);

        let mut destination_label = self.current_label;
        loop {
            destination_label -= 1;
            if destination_label == 0 {
                destination_label = total_len;
            }
            if r1 != destination_label && r2 != destination_label && r3 != destination_label {
                break;
            }
        }
        self.insert(destination_label, r3);
        self.insert(destination_label, r2);
        self.insert(destination_label, r1);
        self.current_label = self.items.get(&self.current_label).unwrap().next;
    }
}
