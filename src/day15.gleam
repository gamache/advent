import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/result
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
