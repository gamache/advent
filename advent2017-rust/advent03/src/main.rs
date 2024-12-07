use std::collections::HashMap;

/*

There is a mathematical solution to this -- it's very similar to an older
problem I've seen, which wanted people to solve for the values at the corner
of this spiral square.

But to make this a better Rust problem, I'm gonna try a state-machiney thing.

*/

#[derive(Hash, Eq, PartialEq, Clone)]
struct Pair {
  x: i32,
  y: i32,
}

struct Spiral {
  i: i32, // current index

  cur: Pair, // current coordinate
  dir: Pair, // direction vector

  max: Pair, // top right corner
  min: Pair, // bottom left corner

  value: i32, // value of current cell
  values: HashMap<Pair,i32>,
}

impl Default for Spiral {
  fn default() -> Spiral {
    Spiral {
      i: 1,
      value: 1,
      cur: Pair { x: 0, y: 0 },
      dir: Pair { x: 1, y: 0 },
      max: Pair { x: 0, y: 0 },
      min: Pair { x: 0, y: 0 },
      values: HashMap::new(),
    }
  }
}

impl Spiral {
  fn manhattan_distance(&self) -> i32 {
    return self.cur.x.abs() + self.cur.y.abs();
  }
}

fn advance_spiral(spiral: &mut Spiral, calculate_values: bool) -> () {
  let new = Pair {
     x: spiral.cur.x + spiral.dir.x,
     y: spiral.cur.y + spiral.dir.y,
  };

  // always turn left
  if new.x > spiral.max.x { // was going right; turn up
    spiral.dir = Pair { x: 0, y: 1 };
    spiral.max.x = new.x;
  }
  else if new.x < spiral.min.x { // was going left; turn down
    spiral.dir = Pair { x: 0, y: -1 };
    spiral.min.x = new.x;
  }
  else if new.y > spiral.max.y { // was going up; turn left
    spiral.dir = Pair { x: -1, y: 0 };
    spiral.max.y = new.y;
  }
  else if new.y < spiral.min.y { // was going down; turn right
    spiral.dir = Pair { x: 1, y: 0 };
    spiral.min.y = new.y;
  }

  if calculate_values {
    // sum all adjacent values
    let adjacent_pairs = [
      Pair { x: new.x, y: new.y },
      Pair { x: new.x, y: new.y+1 },
      Pair { x: new.x, y: new.y-1 },
      Pair { x: new.x+1, y: new.y },
      Pair { x: new.x+1, y: new.y+1 },
      Pair { x: new.x+1, y: new.y-1 },
      Pair { x: new.x-1, y: new.y },
      Pair { x: new.x-1, y: new.y+1 },
      Pair { x: new.x-1, y: new.y-1 },
    ];
    spiral.value = adjacent_pairs
      .iter()
      .flat_map(|pair| spiral.values.get(pair))
      .sum();
    spiral.values.insert(new.clone(), spiral.value);
  }

  spiral.cur = new;
  spiral.i += 1;
}

// Returns the ith spiral.
fn get_spiral_at_index(i: i32) -> Spiral {
  let mut spiral = Spiral { ..Default::default() };
  while spiral.i < i { advance_spiral(&mut spiral, false); }
  return spiral;
}

// Returns the first spiral whose current cell has a value above `value`.
fn get_next_spiral_after_value(value: i32) -> Spiral {
  let mut spiral = Spiral { ..Default::default() };

  // would love to move this line into the default() implementation,
  // but I can't figure out how
  spiral.values.insert(spiral.cur.clone(), spiral.value);

  while spiral.value < value { advance_spiral(&mut spiral, true); }
  return spiral;
}


fn main() {
  let input = 368078;
  println!("Part 1 answer: {}", get_spiral_at_index(input).manhattan_distance());
  println!("Part 2 answer: {}", get_next_spiral_after_value(input).value);
}
