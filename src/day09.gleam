import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/order
import gleam/result
import gleam/set

type Direction {
  Up
  Down
  Left
  Right
}

type Bridge {
  Bridge(
    head_x: Int,
    head_y: Int,
    tail_x: Int,
    tail_y: Int,
    tail_positions: set.Set(#(Int, Int)),
  )
}

fn new_bridge() {
  Bridge(head_x: 0, head_y: 0, tail_x: 0, tail_y: 0, tail_positions: set.new())
}

fn input() -> List(Direction) {
  assert Ok(str) = file.read("inputs/day09.txt")

  str
  |> string.trim
  |> string.split("\n")
  |> list.flat_map(fn(pair) {
    assert [letter, number_str] = string.split(pair, " ")
    assert Ok(number) = int.parse(number_str)
    assert Ok(direction) = letter_to_direction(letter)
    list.repeat(direction, number)
  })
}

fn letter_to_direction(letter: String) -> Result(Direction, Nil) {
  case letter {
    "U" -> Ok(Up)
    "D" -> Ok(Down)
    "L" -> Ok(Left)
    "R" -> Ok(Right)
    _ -> Error(Nil)
  }
}

fn move_head(bridge: Bridge, direction: Direction) -> Bridge {
  case direction {
    Up -> Bridge(..bridge, head_y: bridge.head_y + 1)
    Down -> Bridge(..bridge, head_y: bridge.head_y - 1)
    Left -> Bridge(..bridge, head_x: bridge.head_x - 1)
    Right -> Bridge(..bridge, head_x: bridge.head_x + 1)
  }
}

fn avg(a, b) {
  { a + b } / 2
}

fn move_tail(bridge: Bridge) -> Bridge {
  let dx = int.absolute_value(bridge.head_x - bridge.tail_x)
  let dy = int.absolute_value(bridge.head_y - bridge.tail_y)

  case dx, dy {
    // If the head is less than 2 steps from the tail in any direction, don't move
    x, y if x < 2 && y < 2 -> bridge
    // 2 spaces away horizontal
    x, y if x == 2 && y < 2 -> {
      let new_x = avg(bridge.head_x, bridge.tail_x)
      Bridge(..bridge, tail_x: new_x, tail_y: bridge.head_y)
    }
    // 2 spaces away vertical
    x, y if x < 2 && y == 2 -> {
      let new_y = avg(bridge.head_y, bridge.tail_y)
      Bridge(..bridge, tail_x: bridge.head_x, tail_y: new_y)
    }
    _, _ -> {
      io.println("wtf")
      bridge
    }
  }
  |> record_tail_position
}

fn record_tail_position(bridge: Bridge) -> Bridge {
  let new_tail_positions =
    bridge.tail_positions
    |> set.insert(#(bridge.tail_x, bridge.tail_y))
  Bridge(..bridge, tail_positions: new_tail_positions)
}

fn run(bridge: Bridge, directions: List(Direction)) -> Bridge {
  case directions {
    [] -> bridge
    [dir, ..rest] ->
      bridge
      |> move_head(dir)
      |> move_tail
      |> run(rest)
  }
}

pub fn part1() {
  let bridge = run(new_bridge(), input())
  bridge.tail_positions
  |> set.size()
  |> int.to_string
  |> io.println
}
