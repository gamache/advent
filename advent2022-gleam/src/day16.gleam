import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/regex
import gleam/option

type Cave {
  Cave(flow_rates: map.Map(String, Int), tunnels: map.Map(String, List(String)))
}

fn inspect(v) {
  v
  |> string.inspect
  |> io.println
  v
}

fn input() -> Cave {
  assert Ok(str) = file.read("inputs/day16.txt")

  assert Ok(valve_re) =
    regex.from_string(
      "Valve (..) has flow rate=(\\d+); tunnels? leads? to valves? (.+)",
    )

  str
  |> string.trim
  |> string.split("\n")
  |> list.fold(
    Cave(map.new(), map.new()),
    fn(acc, line) {
      assert [regex.Match(content: _, submatches: submatches)] =
        regex.scan(valve_re, line)

      assert [
        option.Some(valve),
        option.Some(flow_rate_str),
        option.Some(tunnels_str),
      ] = submatches

      assert Ok(flow_rate) = int.parse(flow_rate_str)
      let tunnels = string.split(tunnels_str, ", ")

      Cave(
        flow_rates: map.insert(acc.flow_rates, valve, flow_rate),
        tunnels: map.insert(acc.tunnels, valve, tunnels),
      )
    },
  )
}

type Path {
  Path(
    flow_rate: Int,
    total_flow: Int,
    current_valve: String,
    valves_open: List(String),
  )
}

fn get_next_paths(cave: Cave, path: Path) -> List(Path) {
  let open_valve_path = case
    list.contains(path.valves_open, path.current_valve)
  {
    True -> []
    False -> {
      assert Ok(flow_rate) = map.get(cave.flow_rates, path.current_valve)
      case flow_rate {
        // Don't bother to open a valve with no flow rate
        0 -> []
        _ -> [
          Path(
            ..path,
            flow_rate: path.flow_rate + flow_rate,
            total_flow: path.total_flow + path.flow_rate,
            valves_open: [path.current_valve, ..path.valves_open],
          ),
        ]
      }
    }
  }

  assert Ok(tunnels) = map.get(cave.tunnels, path.current_valve)

  let tunnel_paths =
    list.map(
      tunnels,
      fn(tunnel) {
        Path(
          ..path,
          total_flow: path.total_flow + path.flow_rate,
          current_valve: tunnel,
        )
      },
    )

  list.append(open_valve_path, tunnel_paths)
}

fn dfs(
  cave: Cave,
  paths: List(Path),
  visited: map.Map(String, Int),
  minutes_left: Int,
) -> List(Path) {
  case minutes_left {
    0 -> paths
    _ -> {
      let next_paths =
        paths
        |> list.flat_map(fn(path) { get_next_paths(cave, path) })
        |> list.filter(fn(path: Path) {
          case map.get(visited, path.current_valve) {
            Ok(value) -> value <= path.total_flow
            _ -> True
          }
        })
      let next_visited =
        next_paths
        |> list.fold(
          visited,
          fn(acc, path) {
            let visited_flow = case map.get(acc, path.current_valve) {
              Ok(v) -> v
              _ -> 0
            }
            map.insert(
              acc,
              path.current_valve,
              int.max(visited_flow, path.total_flow),
            )
          },
        )
      dfs(cave, next_paths, next_visited, minutes_left - 1)
    }
  }
}

fn debug_length(xs) {
  xs
  |> list.length
  |> int.to_string
  |> io.println
  xs
}

pub fn part1() {
  let start_path =
    Path(flow_rate: 0, total_flow: 0, current_valve: "AA", valves_open: [])

  dfs(input(), [start_path], map.new(), 30)
  |> list.sort(fn(a, b) { int.compare(b.total_flow, a.total_flow) })
  |> list.first
  |> inspect

  Nil
}
