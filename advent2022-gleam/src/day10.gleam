import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/result
import grid

type Op {
  Noop
  Addx(Int)
}

fn string_to_op(str: String) -> Result(Op, Nil) {
  case str {
    "noop" -> Ok(Noop)
    "addx " <> value_str ->
      value_str
      |> int.parse
      |> result.then(fn(value) { Ok(Addx(value)) })
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

fn run_part2(grid: grid.Grid, cpu: CPU, ops: List(Op)) -> grid.Grid {
  case ops {
    [] -> grid
    [op, ..rest_ops] ->
      case op {
        Noop -> {
          assert #(grid, cpu) = tick_part2(grid, cpu)
          run_part2(grid, cpu, rest_ops)
        }
        Addx(value) -> {
          assert #(grid, cpu) = tick_part2(grid, cpu)
          assert #(grid, cpu) = tick_part2(grid, cpu)
          run_part2(grid, CPU(..cpu, x: cpu.x + value), rest_ops)
        }
      }
  }
}

fn tick_part2(grid: grid.Grid, cpu: CPU) -> #(grid.Grid, CPU) {
  let cycle = cpu.cycle + 1

  assert Ok(x_pos) = int.modulo(cycle, 40)
  let y_pos = cycle / 40

  let x_min = x_pos - 2
  let x_max = x_pos

  let grid = case cpu.x {
    x if x >= x_min && x <= x_max ->
      grid.Grid(..grid, map: map.insert(grid.map, #(y_pos, x_pos), "#"))
    _ -> grid
  }

  let cpu = CPU(..cpu, cycle: cycle)

  #(grid, cpu)
}

pub fn part2() {
  grid.Grid(map: map.new(), colmax: 39, rowmax: 5)
  |> run_part2(new_cpu(), input())
  |> grid.to_string
  |> io.println
}
