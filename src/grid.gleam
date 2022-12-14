// Please welcome back Grid, from day 5

import gleam/string
import gleam/list
import gleam/int
import gleam/map

pub type Grid {
  Grid(map: map.Map(#(Int, Int), String), rowmax: Int, colmax: Int)
}

pub fn new_grid() -> Grid {
  Grid(map.new(), rowmax: 0, colmax: 0)
}

pub fn from_string(str: String) -> Grid {
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

pub fn to_string(grid: Grid) -> String {
  list.range(0, grid.rowmax)
  |> list.map(fn(row) {
    list.range(0, grid.colmax)
    |> list.map(fn(col) {
      case map.get(grid.map, #(row, col)) {
        Ok(str) -> str
        _ -> "."
      }
    })
    |> string.join("")
  })
  |> string.join("\n")
}
