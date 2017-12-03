
/*

There is a mathematical solution to this -- it's very similar to an older
problem I've seen, which wanted people to solve for the values at the corner
of this spiral square.

But to make this a better Rust problem, I'm gonna try a state-machiney thing.

17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...

*/

struct Pair {
  x: i32,
  y: i32,
}

struct Spiral {
  i: i32, // kinda wish the spirals were 0-based but whatever

  cur: Pair, // current coordinate
  dir: Pair, // direction vector

  // extents
  xmax: i32,
  xmin: i32,
  ymax: i32,
  ymin: i32,
}

impl Default for Spiral {
  fn default() -> Spiral {
    Spiral {
      i: 1,
      cur: Pair { x: 0, y: 0 },
      dir: Pair { x: 1, y: 0 },
      xmax: 0, xmin: 0, ymax: 0, ymin: 0,
    }
  }
}

impl Spiral {
  fn manhattan_distance(&self) -> i32 {
    return self.cur.x.abs() + self.cur.y.abs();
  }
}

fn get_spiral(i: i32) -> Spiral {
  let mut spiral = Spiral { ..Default::default() };
  while spiral.i < i {
    let xnew = spiral.cur.x + spiral.dir.x;
    let ynew = spiral.cur.y + spiral.dir.y;

    // always turn left
    if xnew > spiral.xmax { // was going right; turn up
      spiral.dir = Pair { x: 0, y: 1 };
      spiral.xmax = xnew;
    }
    else if xnew < spiral.xmin { // was going left; turn down
      spiral.dir = Pair { x: 0, y: -1 };
      spiral.xmin = xnew;
    }
    else if ynew > spiral.ymax { // was going up; turn left
      spiral.dir = Pair { x: -1, y: 0 };
      spiral.ymax = ynew;
    }
    else if ynew < spiral.ymin { // was going down; turn right
      spiral.dir = Pair { x: 1, y: 0 };
      spiral.ymin = ynew;
    }

    spiral.cur = Pair { x: xnew, y: ynew };
    spiral.i += 1;
  }
  return spiral;
}



fn main() {
  let input = 368078;
  println!("Part 1 answer: {}", get_spiral(input).manhattan_distance());
}
