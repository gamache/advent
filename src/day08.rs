use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop { v: i32 },
    Acc { v: i32 },
    Jmp { v: i32 },
}

#[derive(Debug)]
enum ConsoleState {
    Run,
    InfiniteLoop,
    Halt,
}

struct Console {
    instructions: Vec<Instruction>,
    visited_instructions: HashSet<usize>,
    acc: i32,
    pc: usize,
    state: ConsoleState,
}

impl Console {
    fn new(instructions: Vec<Instruction>) -> Console {
        Console {
            instructions: instructions,
            visited_instructions: HashSet::new(),
            acc: 0,
            pc: 0,
            state: ConsoleState::Run,
        }
    }

    fn reset(&mut self) -> () {
        self.visited_instructions = HashSet::new();
        self.acc = 0;
        self.pc = 0;
        self.state = ConsoleState::Run;
    }

    fn tick(&mut self, munge_instruction_at: usize) -> () {
        let instruction_count = self.instructions.len();

        if self.pc == instruction_count {
            self.state = ConsoleState::Halt;
            return;
        }

        if self.visited_instructions.contains(&self.pc) {
            self.state = ConsoleState::InfiniteLoop;
            return;
        }

        self.state = ConsoleState::Run;
        self.visited_instructions.insert(self.pc);

        let mut instruction = self.instructions[self.pc];
        if self.pc == munge_instruction_at {
            instruction = match instruction {
                Instruction::Acc { v: _ } => instruction,
                Instruction::Jmp { v } => Instruction::Nop { v },
                Instruction::Nop { v } => Instruction::Jmp { v },
            };
        }

        match instruction {
            Instruction::Nop { v: _ } => {
                self.pc = self.pc + 1;
            }
            Instruction::Acc { v } => {
                self.acc = self.acc + v;
                self.pc = self.pc + 1;
            }
            Instruction::Jmp { v } => {
                self.pc = (self.pc as i32 + v).try_into().unwrap();
            }
        };
    }

    fn run(&mut self, munge_instruction_at: usize) -> () {
        self.tick(munge_instruction_at);
        match self.state {
            ConsoleState::Run => self.run(munge_instruction_at),
            _ => {}
        };
    }
}

pub fn run(lines: Vec<String>) -> () {
    let mut console = Console::new(lines_to_instructions(&lines));
    part1(&mut console);
    part2(&mut console);
}

fn part1(console: &mut Console) -> () {
    console.run(console.instructions.len());
    println!("{}", console.acc);
}

fn part2(console: &mut Console) -> () {
    let mut munge: usize = 0;
    let munge_max = console.instructions.len();
    while munge < munge_max {
        console.reset();
        console.run(munge);
        match console.state {
            ConsoleState::Halt => {
                println!("{}", console.acc);
                return;
            }
            _ => {}
        };
        munge = munge + 1;
    }
}

fn lines_to_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let v: i32 = match parts.get(1) {
            None => 0,
            Some(vstr) => vstr.parse::<i32>().unwrap(),
        };
        match parts[0] {
            "nop" => instructions.push(Instruction::Nop { v }),
            "acc" => instructions.push(Instruction::Acc { v }),
            "jmp" => instructions.push(Instruction::Jmp { v }),
            _ => panic!("bad opcode: {}", parts[0]),
        }
    }
    instructions
}
