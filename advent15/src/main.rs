const GEN_A_START: i64 = 722;
const GEN_B_START: i64 = 354;

const GEN_A_FACTOR: i64 = 16807;
const GEN_B_FACTOR: i64 = 48271;

const GEN_A_DIVISOR: i64 = 4;
const GEN_B_DIVISOR: i64 = 8;

const DENOMINATOR: i64 = 2147483647;

fn main() {
  println!("Part 1 answer: {}", count_matches(40_000_000));
  println!("Part 2 answer: {}", count_matches2(5_000_000));
}

struct Generator {
  value: i64,
  factor: i64,
  divisor: i64,
}

impl Generator {
  fn tick(&mut self) -> () {
    self.value = (self.value * self.factor) % DENOMINATOR;
  }

  fn tick2(&mut self) -> () {
    let mut divisible = false;
    while !divisible {
      self.tick();
      if self.value % self.divisor == 0 {
        divisible = true;
      }
    }
  }
}

fn count_matches(n: usize) -> usize {
  let mut matches = 0;
  let mut gen_a = Generator { value: GEN_A_START, factor: GEN_A_FACTOR, divisor: GEN_A_DIVISOR };
  let mut gen_b = Generator { value: GEN_B_START, factor: GEN_B_FACTOR, divisor: GEN_B_DIVISOR };
  for i in 1..n {
    gen_a.tick();
    gen_b.tick();
    //println!("{} {} {}", i, gen_a.value, gen_b.value);
    if (gen_a.value & 0xFFFF) == (gen_b.value & 0xFFFF) {
      matches += 1;
    }
  }
  return matches;
}

fn count_matches2(n: usize) -> usize {
  let mut matches = 0;
  let mut gen_a = Generator { value: GEN_A_START, factor: GEN_A_FACTOR, divisor: GEN_A_DIVISOR };
  let mut gen_b = Generator { value: GEN_B_START, factor: GEN_B_FACTOR, divisor: GEN_B_DIVISOR };
  for i in 1..n {
    gen_a.tick2();
    gen_b.tick2();
    //println!("{} {} {}", i, gen_a.value, gen_b.value);
    if (gen_a.value & 0xFFFF) == (gen_b.value & 0xFFFF) {
      matches += 1;
    }
  }
  return matches;
}
