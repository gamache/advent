import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int

fn input() {
  assert Ok(input_str) =
    "inputs/day04.txt"
    |> file.read()

  input_str
  |> string.trim()
  |> string.split("\n")
  |> list.map(fn(ranges) {
    assert [Ok(x1), Ok(x2), Ok(y1), Ok(y2)] =
      ranges
      |> string.split(",")
      |> list.flat_map(string.split(_, "-"))
      |> list.map(int.parse)

    let xlist = list.range(x1, x2)
    let ylist = list.range(y1, y2)

    #(xlist, ylist)
  })
}

fn is_subset(list, sublist) {
  sublist
  |> list.all(fn(item) { list.contains(list, item) })
}

pub fn part1() {
  input()
  |> list.filter(fn(lists) {
    assert #(xlist, ylist) = lists
    is_subset(xlist, ylist) || is_subset(ylist, xlist)
  })
  |> list.length()
  |> int.to_string()
  |> io.println()
}

fn overlaps(xlist, ylist) {
  xlist
  |> list.any(fn(item) { list.contains(ylist, item) })
}

pub fn part2() {
  input()
  |> list.filter(fn(lists) {
    assert #(xlist, ylist) = lists
    overlaps(xlist, ylist) || overlaps(ylist, xlist)
  })
  |> list.length()
  |> int.to_string()
  |> io.println()
}
