import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/order
import gleam/result
import gleam/regex
import gleam/option

pub fn part1() {
  assert #(stacks_map, moves) = input()

  moves
  |> list.fold(stacks_map, apply_move_part1)
  |> print_top_of_stacks()
}

pub fn part2() {
  assert #(stacks_map, moves) = input()

  moves
  |> list.fold(stacks_map, apply_move_part2)
  |> print_top_of_stacks()
}

fn print_top_of_stacks(stacks_map) {
  list.range(1, 9)
  |> list.map(fn(stack_number) { map.get(stacks_map, stack_number) })
  |> result.values()
  |> list.map(fn(stack) {
    stack
    |> list.first
    |> result.unwrap(" ")
    |> io.print
  })

  io.println("")
}

fn input() {
  assert Ok(input_str) =
    "inputs/day05.txt"
    |> file.read()

  assert [stacks_str, moves_str] =
    input_str
    |> string.trim_right()
    |> string.split("\n\n")

  let stacks_map = parse_stacks(stacks_str)
  let moves = parse_moves(moves_str)

  #(stacks_map, moves)
}

type StacksMap =
  map.Map(Int, List(String))

fn parse_stacks(stacks_str: String) -> StacksMap {
  let grid = string_to_grid(stacks_str)

  list.range(1, 9)
  |> list.fold(
    map.new(),
    fn(acc, stack_number) {
      map.insert(acc, stack_number, get_stack(grid, stack_number))
    },
  )
}

fn get_stack(grid: Grid, stack_number: Int) -> List(String) {
  let col = -3 + 4 * stack_number

  list.range(0, grid.rowmax)
  |> list.map(fn(row) {
    grid.map
    |> map.get(#(row, col))
    |> result.unwrap(" ")
    |> letter
  })
  |> result.values()
}

fn letter(char: String) -> Result(String, Nil) {
  case string.compare(char, "A"), string.compare(char, "Z") {
    order.Lt, _ -> Error(Nil)
    _, order.Gt -> Error(Nil)
    _, _ -> Ok(char)
  }
}

type Move {
  Move(count: Int, from: Int, to: Int)
}

fn parse_moves(moves_str: String) -> List(Move) {
  assert Ok(move_regex) = regex.from_string("move (\\d+) from (\\d+) to (\\d+)")

  moves_str
  |> string.split("\n")
  |> list.map(fn(line) {
    assert [
      regex.Match(
        content: _,
        submatches: [
          option.Some(count_str),
          option.Some(from_str),
          option.Some(to_str),
        ],
      ),
    ] = regex.scan(move_regex, line)

    assert Ok(count) = int.parse(count_str)
    assert Ok(from) = int.parse(from_str)
    assert Ok(to) = int.parse(to_str)

    Move(count: count, from: from, to: to)
  })
}

fn apply_move_part1(stacks_map: StacksMap, move: Move) -> StacksMap {
  assert Ok(from_stack) = map.get(stacks_map, move.from)
  assert Ok(to_stack) = map.get(stacks_map, move.to)

  assert #(popped, new_from_stack) = list.split(from_stack, move.count)

  let new_to_stack =
    popped
    |> list.reverse
    |> list.append(to_stack)

  stacks_map
  |> map.insert(move.from, new_from_stack)
  |> map.insert(move.to, new_to_stack)
}

fn apply_move_part2(stacks_map: StacksMap, move: Move) -> StacksMap {
  assert Ok(from_stack) = map.get(stacks_map, move.from)
  assert Ok(to_stack) = map.get(stacks_map, move.to)

  assert #(popped, new_from_stack) = list.split(from_stack, move.count)

  let new_to_stack =
    popped
    |> list.append(to_stack)

  stacks_map
  |> map.insert(move.from, new_from_stack)
  |> map.insert(move.to, new_to_stack)
}

// I have done enough Advents of Code to know that I am very likely
// to reuse this in the coming days. Behold, a map of #(row, col)
// coordinates to single-character strings.
type Grid {
  Grid(map: map.Map(#(Int, Int), String), rowmax: Int, colmax: Int)
}

fn string_to_grid(str: String) -> Grid {
  let the_map =
    str
    |> string.split("\n")
    |> list.index_fold(
      map.new(),
      fn(acc, line, row) {
        //io.println("")
        line
        |> string.to_graphemes()
        |> list.index_fold(
          acc,
          fn(acc2, char, col) {
            //io.print(char)
            map.insert(acc2, #(row, col), char)
          },
        )
      },
    )

  assert #(rowmax, colmax) =
    the_map
    |> map.keys()
    |> list.fold(
      #(0, 0),
      fn(acc, rowcol) {
        assert #(row, col) = rowcol
        assert #(rmax, cmax) = acc
        #(int.max(rmax, row), int.max(cmax, col))
      },
    )

  Grid(map: the_map, rowmax: rowmax, colmax: colmax)
}
