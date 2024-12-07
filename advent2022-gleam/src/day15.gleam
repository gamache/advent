import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/regex
import gleam/option

type Sensor {
  Sensor(x: Int, y: Int, beacon_x: Int, beacon_y: Int, beacon_distance: Int)
}

fn input() -> List(Sensor) {
  assert Ok(str) = file.read("inputs/day15.txt")

  assert Ok(sensor_re) =
    regex.from_string(
      "Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)",
    )

  str
  |> string.trim
  |> string.split("\n")
  |> list.map(fn(sensor_str) {
    case regex.scan(sensor_re, sensor_str) {
      [regex.Match(content: _, submatches: submatches)] -> {
        assert [sensor_x, sensor_y, beacon_x, beacon_y] =
          submatches
          |> list.map(fn(sm) {
            assert option.Some(str) = sm
            assert Ok(i) = int.parse(str)
            i
          })
        Sensor(
          x: sensor_x,
          y: sensor_y,
          beacon_x: beacon_x,
          beacon_y: beacon_y,
          beacon_distance: distance(sensor_x, sensor_y, beacon_x, beacon_y),
        )
      }
    }
  })
}

fn distance(x1: Int, y1: Int, x2: Int, y2: Int) -> Int {
  int.absolute_value(x1 - x2) + int.absolute_value(y1 - y2)
}

fn too_close(sensor: Sensor, x: Int, y: Int) -> Bool {
  sensor.beacon_distance >= distance(sensor.x, sensor.y, x, y)
}

pub fn part1() {
  let sensors = input()

  let y = 2000000

  assert Ok(xmin) =
    sensors
    |> list.map(fn(s) { int.min(s.x, s.beacon_x) })
    |> list.reduce(int.min)

  assert Ok(xmax) =
    sensors
    |> list.map(fn(s) { int.max(s.x, s.beacon_x) })
    |> list.reduce(int.max)

  assert distancemax =
    list.fold(sensors, 0, fn(acc, s) { int.max(acc, s.beacon_distance) })

  list.range(xmin - distancemax, xmax + distancemax)
  |> list.filter(fn(x) {
    list.any(
      sensors,
      fn(s) { too_close(s, x, y) && { s.beacon_x != x || s.beacon_y != y } },
    )
  })
  |> list.length
  |> int.to_string
  |> io.println
}

fn find_possible_beacon(
  sensors: List(Sensor),
  x: Int,
  y: Int,
  x_max: Int,
  y_max: Int,
) -> Result(Int, Nil) {
  case x > x_max, y > y_max {
    True, True -> Error(Nil)
    True, False -> find_possible_beacon(sensors, 0, y + 1, x_max, y_max)
    False, _ -> {
      let next_x =
        list.fold(
          sensors,
          x,
          fn(acc, sensor) {
            let dy = int.absolute_value(sensor.y - y)
            let max_dx = int.max(0, sensor.beacon_distance - dy)
            case int.absolute_value(sensor.x - x) {
              dx if dx > max_dx -> acc
              _ -> int.max(sensor.x + max_dx, acc)
            }
          },
        )
      case next_x == x {
        True -> Ok(4000000 * x + y)
        False -> find_possible_beacon(sensors, next_x + 1, y, x_max, y_max)
      }
    }
  }
}

pub fn part2() {
  assert Ok(i) = find_possible_beacon(input(), 0, 0, 4000000, 4000000)

  i
  |> int.to_string
  |> io.println
}
