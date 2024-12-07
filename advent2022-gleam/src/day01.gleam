import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/result
import gleam/int

pub fn part1() {
  let [biggest, ..] = input()

  biggest
  |> int.to_string()
  |> io.println()
}

pub fn part2() {
  let [x1, x2, x3, ..] = input()

  x1 + x2 + x3
  |> int.to_string()
  |> io.println()
}

fn input() {
  "inputs/day01.txt"
  |> file.read()
  |> result.unwrap("nope")
  |> string.trim()
  |> string.split("\n\n")
  |> list.map(fn(chunk) {
    chunk
    |> string.split("\n")
    |> list.map(int.parse)
    |> result.values()
    |> list.reduce(fn(acc, x) { acc + x })
  })
  |> result.values()
  |> list.sort(by: int.compare)
  |> list.reverse()
}
