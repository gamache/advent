import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/set
import gleam/result

// some ascii values
const start = 83

// nice
const end = 69

const lower_a = 97

const lower_z = 122

type SearchPath {
  SearchPath(grid: Grid, path: List(#(Int, Int)), length: Int, complete: Bool)
}

fn next_move(
  search_path: SearchPath,
  height: Int,
  coord: #(Int, Int),
) -> Result(SearchPath, Nil) {
  let path = search_path.path
  let length = search_path.length
  let max_height = height + 1

  case list.contains(search_path.path, coord) {
    True -> Error(Nil)
    False ->
      case map.get(search_path.grid.map, coord) {
        Ok(h) if h == end ->
          Ok(
            SearchPath(
              ..search_path,
              path: [coord, ..path],
              length: length + 1,
              complete: True,
            ),
          )
        Ok(h) if h <= max_height ->
          Ok(
            SearchPath(..search_path, path: [coord, ..path], length: length + 1),
          )
        _ -> Error(Nil)
      }
  }
}

fn next_paths_part1(search_path: SearchPath) -> List(SearchPath) {
  assert [#(row, col), ..] = search_path.path
  assert Ok(height) = case map.get(search_path.grid.map, #(row, col)) {
    Ok(h) if h == start -> Ok(lower_a)
    Ok(h) -> Ok(h)
    _ -> Error(Nil)
  }

  [
    next_move(search_path, height, #(row, col - 1)),
    next_move(search_path, height, #(row, col + 1)),
    next_move(search_path, height, #(row - 1, col)),
    next_move(search_path, height, #(row + 1, col)),
  ]
  |> result.values
}

fn path_coord(search_path: SearchPath) -> #(Int, Int) {
  assert [coord, ..] = search_path.path
  coord
}

fn bfs(
  search_paths: List(SearchPath),
  visited: set.Set(#(Int, Int)),
  next_paths_fn: fn(SearchPath) -> List(SearchPath),
) -> SearchPath {
  let next_paths: List(SearchPath) =
    search_paths
    |> list.flat_map(next_paths_fn)
    |> list.filter(fn(sp) { !set.contains(visited, path_coord(sp)) })
    |> list.fold(map.new(), fn(acc, sp) { map.insert(acc, path_coord(sp), sp) })
    |> map.values

  case list.filter(next_paths, fn(sp) { sp.complete }) {
    [winner, ..] -> winner
    _ -> {
      let visited =
        next_paths
        |> list.map(path_coord)
        |> list.fold(visited, fn(acc, coord) { set.insert(acc, coord) })
      bfs(next_paths, visited, next_paths_fn)
    }
  }
}

fn find(grid: Grid, height: Int) -> SearchPath {
  let coords =
    list.range(0, grid.rowmax)
    |> list.flat_map(fn(row) {
      list.range(0, grid.colmax)
      |> list.map(fn(col) { #(row, col) })
    })

  assert [start_coord] =
    list.filter(coords, fn(coord) { Ok(height) == map.get(grid.map, coord) })

  SearchPath(grid: grid, path: [start_coord], length: 0, complete: False)
}

pub fn part1() {
  let first_path = find(input(), start)
  let visited = set.insert(set.new(), path_coord(first_path))
  let winner = bfs([first_path], visited, next_paths_part1)

  winner.length
  |> int.to_string
  |> io.println
}

fn next_paths_part2(search_path: SearchPath) -> List(SearchPath) {
  assert [#(row, col), ..] = search_path.path
  assert Ok(height) = case map.get(search_path.grid.map, #(row, col)) {
    Ok(h) if h == end -> Ok(lower_z)
    Ok(h) if h == start -> Ok(lower_a)
    Ok(h) -> Ok(h)
    _ -> Error(Nil)
  }

  [
    next_move_part2(search_path, height, #(row, col - 1)),
    next_move_part2(search_path, height, #(row, col + 1)),
    next_move_part2(search_path, height, #(row - 1, col)),
    next_move_part2(search_path, height, #(row + 1, col)),
  ]
  |> result.values
}

fn next_move_part2(
  search_path: SearchPath,
  height: Int,
  coord: #(Int, Int),
) -> Result(SearchPath, Nil) {
  let path = search_path.path
  let length = search_path.length
  let min_height = height - 1

  case list.contains(search_path.path, coord) {
    True -> Error(Nil)
    False ->
      case map.get(search_path.grid.map, coord) {
        Ok(h) if h >= min_height && h == lower_a ->
          Ok(
            SearchPath(
              ..search_path,
              path: [coord, ..path],
              length: length + 1,
              complete: True,
            ),
          )
        Ok(h) if h >= min_height ->
          Ok(
            SearchPath(..search_path, path: [coord, ..path], length: length + 1),
          )
        _ -> Error(Nil)
      }
  }
}

pub fn part2() {
  let first_path = find(input(), end)
  let visited = set.insert(set.new(), path_coord(first_path))
  let winner = bfs([first_path], visited, next_paths_part2)

  winner.length
  |> int.to_string
  |> io.println
}

fn input() -> Grid {
  assert Ok(str) = file.read("inputs/day12.txt")
  str
  |> string.trim
  |> grid_from_string
}

external fn binary_to_list(x: String) -> List(Int) =
  "erlang" "binary_to_list"

type Grid {
  Grid(map: map.Map(#(Int, Int), Int), rowmax: Int, colmax: Int)
}

fn grid_from_string(str: String) -> Grid {
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
            // map.insert(acc2, #(row, col), char)
            assert [char_value, ..] = binary_to_list(char)
            map.insert(acc2, #(row, col), char_value)
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
