import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/order
import gleam/result

fn input() -> Grid {
  assert Ok(str) = file.read("inputs/day08.txt")
  string_to_grid(str)
}

fn visible(grid: Grid, row: Int, col: Int) -> Bool {
  assert Ok(tree) = map.get(grid.map, #(row, col))
  let rowmax = grid.rowmax
  let colmax = grid.colmax

  let from_top = case row {
    0 -> True
    _ ->
      list.range(0, row - 1)
      |> list.all(fn(r) {
        assert Ok(t) = map.get(grid.map, #(r, col))
        order.Lt == string.compare(t, tree)
      })
  }

  let from_bottom = case row {
    // You can't say "r if r == grid.rowmax" here, oddly
    r if r == rowmax -> True
    _ ->
      list.range(row + 1, grid.rowmax)
      |> list.all(fn(r) {
        assert Ok(t) = map.get(grid.map, #(r, col))
        order.Lt == string.compare(t, tree)
      })
  }

  let from_left = case col {
    0 -> True
    _ ->
      list.range(0, col - 1)
      |> list.all(fn(c) {
        assert Ok(t) = map.get(grid.map, #(row, c))
        order.Lt == string.compare(t, tree)
      })
  }

  let from_right = case col {
    c if c == colmax -> True
    _ ->
      list.range(col + 1, grid.colmax)
      |> list.all(fn(c) {
        assert Ok(t) = map.get(grid.map, #(row, c))
        order.Lt == string.compare(t, tree)
      })
  }

  from_top || from_bottom || from_left || from_right
}

pub fn part1() {
  let grid = input()
  list.range(0, grid.rowmax)
  |> list.flat_map(fn(row) {
    list.range(0, grid.colmax)
    |> list.map(fn(col) { visible(grid, row, col) })
  })
  |> list.filter(fn(x) { x })
  |> list.length()
  |> int.to_string()
  |> io.println
}

fn scenic_score(grid: Grid, row: Int, col: Int) -> Int {
  assert Ok(tree) = map.get(grid.map, #(row, col))
  let rowmax = grid.rowmax
  let colmax = grid.colmax

  let top_scenic = case row {
    0 -> 0
    _ ->
      int.min(
        {
          list.range(0, row - 1)
          |> list.reverse
          |> list.take_while(fn(r) {
            assert Ok(t) = map.get(grid.map, #(r, col))
            order.Lt == string.compare(t, tree)
          })
          |> list.length
        } + 1,
        row,
      )
  }

  let bottom_scenic = case row {
    r if r == rowmax -> 0
    _ ->
      int.min(
        {
          list.range(row + 1, rowmax)
          |> list.take_while(fn(r) {
            assert Ok(t) = map.get(grid.map, #(r, col))
            order.Lt == string.compare(t, tree)
          })
          |> list.length
        } + 1,
        rowmax - row,
      )
  }

  let left_scenic = case col {
    0 -> 0
    _ ->
      int.min(
        {
          list.range(0, col - 1)
          |> list.reverse
          |> list.take_while(fn(c) {
            assert Ok(t) = map.get(grid.map, #(row, c))
            order.Lt == string.compare(t, tree)
          })
          |> list.length
        } + 1,
        col,
      )
  }

  let right_scenic = case col {
    c if c == colmax -> 0
    _ ->
      int.min(
        {
          list.range(col + 1, colmax)
          |> list.take_while(fn(c) {
            assert Ok(t) = map.get(grid.map, #(row, c))
            order.Lt == string.compare(t, tree)
          })
          |> list.length
        } + 1,
        colmax - col,
      )
  }

  top_scenic * bottom_scenic * left_scenic * right_scenic
}

pub fn part2() {
  let grid = input()

  list.range(0, grid.rowmax)
  |> list.flat_map(fn(row) {
    list.range(0, grid.colmax)
    |> list.map(fn(col) { scenic_score(grid, row, col) })
  })
  |> list.sort(int.compare)
  |> list.reverse
  |> list.first
  |> result.unwrap(0)
  |> int.to_string
  |> io.println
}

// Please welcome back Grid, from day 5
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
        // io.println("")
        line
        |> string.to_graphemes()
        |> list.index_fold(
          acc,
          fn(acc2, char, col) {
            // io.print(char)
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
