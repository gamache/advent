pub fn run(lines: Vec<String>) {
    let card_pubkey = lines[0].parse::<usize>().unwrap();
    let door_pubkey = lines[1].parse::<usize>().unwrap();
    let mut card_loop_size = 0;
    let mut door_loop_size = 0;
    let mut loop_size = 0;
    let mut value = 1;
    loop {
        loop_size += 1;
        value *= 7;
        value %= 20201227;
        if value == card_pubkey {
            // println!("card_loop_size {}", loop_size);
            card_loop_size = loop_size;
        }
        if value == door_pubkey {
            // println!("door_loop_size {}", loop_size);
            door_loop_size = loop_size;
        }
        if card_loop_size > 0 && door_loop_size > 0 {
            break;
        }
    }
    let encryption_key = transform(card_pubkey, door_loop_size);
    println!("{}", encryption_key);
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}
