import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/set

pub fn input() {
  assert Ok(input_str) =
    "inputs/day03.txt"
    |> file.read()

  input_str
  |> string.trim()
  |> string.split("\n")
  |> list.map(fn(letters) {
    let len = string.length(letters)
    let compartment_size = len / 2

    let letters1 = string.slice(letters, 0, compartment_size)
    let letters2 = string.slice(letters, compartment_size, compartment_size)

    let compartment1 =
      letters1
      |> string.to_graphemes()
      |> set.from_list

    let compartment2 =
      letters2
      |> string.to_graphemes()
      |> set.from_list

    #(compartment1, compartment2)
  })
}

external fn binary_to_list(x: String) -> List(Int) =
  "erlang" "binary_to_list"

// I really want to do something like:
// const [lower_a, lower_z, upper_a, upper_z] = binary_to_list("azAZ")
// but there appears to be no destructuring with consts
const lower_a = 97

const lower_z = 122

const upper_a = 65

const upper_z = 90

fn priority(char) {
  assert [i] = binary_to_list(char)
  case i {
    x if x >= lower_a && x <= lower_z -> x - lower_a + 1
    x if x >= upper_a && x <= upper_z -> x - upper_a + 27
    _ -> 0
  }
}

pub fn part1() {
  assert Ok(sum) =
    input()
    |> list.map(fn(compartments) {
      assert #(comp1, comp2) = compartments
      assert [char] =
        set.intersection(comp1, comp2)
        |> set.to_list()

      priority(char)
    })
    |> list.reduce(fn(acc, x) { acc + x })

  sum
  |> int.to_string
  |> io.println
}

pub fn part2() {
  assert Ok(sum) =
    input()
    |> list.sized_chunk(3)
    |> list.map(fn(chunk) {
      assert [#(c1, c2), #(c3, c4), #(c5, c6)] = chunk
      assert [char] =
        set.union(c1, c2)
        |> set.intersection(set.union(c3, c4))
        |> set.intersection(set.union(c5, c6))
        |> set.to_list()
      priority(char)
    })
    |> list.reduce(fn(acc, x) { acc + x })

  sum
  |> int.to_string
  |> io.println
}
