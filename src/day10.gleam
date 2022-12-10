import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/order
import gleam/result

type Op {
  Noop
  Addx(Int)
}

fn string_to_op(str: String) -> Result(Op, Nil) {
  case str {
    "noop" -> Ok(Noop)
    "addx " <> value_str -> {
      assert Ok(value) = int.parse(value_str)
      Ok(Addx(value))
    }
    _ -> Error(Nil)
  }
}

fn input() -> List(Op) {
  assert Ok(str) = file.read("inputs/day10.txt")

  str
  |> string.trim
  |> string.split("\n")
  |> list.map(string_to_op)
  |> result.values
}

type CPU {
  CPU(x: Int, cycle: Int, strengths: List(Int))
}

fn new_cpu() -> CPU {
  CPU(1, 0, [])
}

fn tick(cpu: CPU) -> CPU {
  let cpu = CPU(..cpu, cycle: cpu.cycle + 1)

  case int.modulo(cpu.cycle + 20, 40) {
    Ok(0) -> CPU(..cpu, strengths: [strength(cpu), ..cpu.strengths])
    _ -> cpu
  }
}

fn strength(cpu: CPU) -> Int {
  cpu.cycle * cpu.x
}

fn run(cpu: CPU, ops: List(Op)) -> CPU {
  case ops {
    [] -> cpu
    [op, ..rest_ops] ->
      case op {
        Noop ->
          cpu
          |> tick
          |> run(rest_ops)
        Addx(value) -> {
          let cpu =
            cpu
            |> tick
            |> tick
          run(CPU(..cpu, x: cpu.x + value), rest_ops)
        }
      }
  }
}

pub fn part1() {
  let cpu = run(new_cpu(), input())

  cpu.strengths
  |> list.reverse
  |> list.take(6)
  |> list.fold(0, fn(acc, x) { acc + x })
  |> int.to_string
  |> io.println
}
