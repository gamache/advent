import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/result
import grid

fn input() -> grid.Grid {
  assert Ok(str) = file.read("inputs/day14.txt")

  str
  |> string.trim
  |> string.split("\n")
  |> list.fold(
    grid.new_grid(),
    fn(grid, rock_path_str) { create_rock_path(grid, rock_path_str) },
  )
}

fn create_rock_path(g: grid.Grid, rock_path_str: String) -> grid.Grid {
  rock_path_str
  |> string.split(" -> ")
  |> list.map(fn(coord_str) {
    assert [col, row] =
      coord_str
      |> string.split(",")
      |> list.map(int.parse)
      |> result.values

    #(row, col)
  })
  |> list.window(2)
  |> list.fold(
    g,
    fn(acc, coords) {
      assert [#(a_row, a_col), #(b_row, b_col)] = coords
      let row_start = int.min(a_row, b_row)
      let row_end = int.max(a_row, b_row)
      let col_start = int.min(a_col, b_col)
      let col_end = int.max(a_col, b_col)

      list.range(row_start, row_end)
      |> list.flat_map(fn(row) {
        list.range(col_start, col_end)
        |> list.map(fn(col) { #(row, col) })
      })
      |> list.fold(
        acc,
        fn(acc2, coord) {
          assert #(row, col) = coord
          let rowmax = int.max(row, acc2.rowmax)
          let colmax = int.max(col, acc2.colmax)
          grid.Grid(
            map: map.insert(acc2.map, coord, "#"),
            rowmax: rowmax,
            colmax: colmax,
          )
        },
      )
    },
  )
}

fn drop_all_sand_part1(g: grid.Grid, count: Int) -> Int {
  case drop_one_sand_part1(g, #(0, 500)) {
    Ok(g) -> drop_all_sand_part1(g, count + 1)
    Error(Nil) -> count
  }
}

fn drop_one_sand_part1(
  g: grid.Grid,
  coord: #(Int, Int),
) -> Result(grid.Grid, Nil) {
  assert #(row, col) = coord

  case row == g.rowmax {
    True -> Error(Nil)
    False -> {
      let below_coord = #(row + 1, col)
      let below_left_coord = #(row + 1, col - 1)
      let below_right_coord = #(row + 1, col + 1)
      case map.get(g.map, below_coord) {
        Ok(_) ->
          case map.get(g.map, below_left_coord) {
            Ok(_) ->
              case map.get(g.map, below_right_coord) {
                Ok(_) -> Ok(grid.Grid(..g, map: map.insert(g.map, coord, "o")))
                _ -> drop_one_sand_part1(g, below_right_coord)
              }
            _ -> drop_one_sand_part1(g, below_left_coord)
          }
        _ -> drop_one_sand_part1(g, below_coord)
      }
    }
  }
}

pub fn part1() {
  input()
  |> drop_all_sand_part1(0)
  |> int.to_string
  |> io.println
}

fn drop_all_sand_part2(g: grid.Grid, count: Int) -> Int {
  case drop_one_sand_part2(g, #(0, 500)) {
    Ok(g) -> drop_all_sand_part2(g, count + 1)
    Error(Nil) -> count
  }
}

fn drop_one_sand_part2(
  g: grid.Grid,
  coord: #(Int, Int),
) -> Result(grid.Grid, Nil) {
  assert #(row, col) = coord

  case row == g.rowmax + 1 {
    True -> Ok(grid.Grid(..g, map: map.insert(g.map, coord, "o")))
    False -> {
      let below_coord = #(row + 1, col)
      let below_left_coord = #(row + 1, col - 1)
      let below_right_coord = #(row + 1, col + 1)
      case map.get(g.map, below_coord) {
        Ok(_) ->
          case map.get(g.map, below_left_coord) {
            Ok(_) ->
              case map.get(g.map, below_right_coord) {
                Ok(_) ->
                  case coord {
                    #(0, 500) -> Error(Nil)
                    _ -> Ok(grid.Grid(..g, map: map.insert(g.map, coord, "o")))
                  }
                _ -> drop_one_sand_part2(g, below_right_coord)
              }
            _ -> drop_one_sand_part2(g, below_left_coord)
          }
        _ -> drop_one_sand_part2(g, below_coord)
      }
    }
  }
}

pub fn part2() {
  input()
  |> drop_all_sand_part2(1)
  |> int.to_string
  |> io.println
}
