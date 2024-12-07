import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/set

fn input() {
  assert Ok(str) =
    "inputs/day06.txt"
    |> file.read()

  string.to_graphemes(str)
}

fn find_start_of_packet(lst, run_length, count_so_far) {
  let head = list.take(lst, run_length)
  let head_set = set.from_list(head)
  case set.size(head_set) {
    x if x < run_length -> {
      assert [_, ..rest] = lst
      find_start_of_packet(rest, run_length, count_so_far + 1)
    }
    _ -> count_so_far
  }
}

pub fn part1() {
  input()
  |> find_start_of_packet(4, 4)
  |> int.to_string
  |> io.println
}

pub fn part2() {
  input()
  |> find_start_of_packet(14, 14)
  |> int.to_string
  |> io.println
}
