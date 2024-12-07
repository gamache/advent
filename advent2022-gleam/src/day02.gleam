import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int

type RPS {
  Rock
  Paper
  Scissors
}

fn letter_to_rps(letter: String) -> Result(RPS, String) {
  case letter {
    "A" -> Ok(Rock)
    "B" -> Ok(Paper)
    "C" -> Ok(Scissors)
    "X" -> Ok(Rock)
    "Y" -> Ok(Paper)
    "Z" -> Ok(Scissors)
    _ -> Error("bad letter: '" <> letter <> "'")
  }
}

/// Returns 0 if they beat us, 3 for a tie, or 6 if we beat them.
fn outcome_score(them: RPS, us: RPS) {
  case them, us {
    Rock, Rock -> 3
    Rock, Paper -> 6
    Rock, Scissors -> 0
    Paper, Rock -> 0
    Paper, Paper -> 3
    Paper, Scissors -> 6
    Scissors, Rock -> 6
    Scissors, Paper -> 0
    Scissors, Scissors -> 3
  }
}

/// Returns the score for a RPS shape.
fn shape_score(shape: RPS) {
  case shape {
    Rock -> 1
    Paper -> 2
    Scissors -> 3
  }
}

/// Returns our score for a single round of RPS.
fn score(them, us) {
  outcome_score(them, us) + shape_score(us)
}

fn input1() {
  assert Ok(input_str) =
    "inputs/day02.txt"
    |> file.read()

  input_str
  |> string.trim()
  |> string.split("\n")
  |> list.map(fn(pair) {
    assert [Ok(them), Ok(us)] =
      pair
      |> string.trim()
      |> string.split(" ")
      |> list.map(letter_to_rps)

    score(them, us)
  })
}

pub fn part1() {
  assert Ok(sum) =
    input1()
    |> list.reduce(fn(acc, x) { acc + x })

  sum
  |> int.to_string
  |> io.println
}

type WLD {
  Win
  Lose
  Draw
}

fn letter_to_wld(letter: String) -> Result(WLD, String) {
  case letter {
    "X" -> Ok(Lose)
    "Y" -> Ok(Draw)
    "Z" -> Ok(Win)
    _ -> Error("bad letter: '" <> letter <> "'")
  }
}

fn wld_to_shape(them: RPS, us_wld: WLD) -> RPS {
  case them, us_wld {
    Rock, Lose -> Scissors
    Rock, Draw -> Rock
    Rock, Win -> Paper
    Paper, Lose -> Rock
    Paper, Draw -> Paper
    Paper, Win -> Scissors
    Scissors, Lose -> Paper
    Scissors, Draw -> Scissors
    Scissors, Win -> Rock
  }
}

fn input2() {
  assert Ok(input_str) =
    "inputs/day02.txt"
    |> file.read()

  input_str
  |> string.trim()
  |> string.split("\n")
  |> list.map(fn(pair) {
    assert [them_str, wld_str] =
      pair
      |> string.trim()
      |> string.split(" ")

    assert Ok(them) = letter_to_rps(them_str)
    assert Ok(wld) = letter_to_wld(wld_str)
    let us = wld_to_shape(them, wld)

    score(them, us)
  })
}

pub fn part2() {
  assert Ok(sum) =
    input2()
    |> list.reduce(fn(acc, x) { acc + x })

  sum
  |> int.to_string
  |> io.println
}
