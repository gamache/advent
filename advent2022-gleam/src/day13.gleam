import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/order

// #(list of chars, list of chars) tuples
type LeftAndRight =
  #(List(String), List(String))

fn input() -> List(LeftAndRight) {
  assert Ok(str) = file.read("inputs/day13.txt")

  str
  |> string.trim
  |> string.split("\n\n")
  |> list.map(fn(pair_str) {
    assert [a, b] =
      pair_str
      |> string.split("\n")
      |> list.map(string.to_graphemes)
    #(a, b)
  })
}

fn correct_order(lr: LeftAndRight) -> Bool {
  assert #(left, right) = lr
  order.Lt == compare_packets(left, right)
}

fn compare_packets(left: List(String), right: List(String)) -> order.Order {
  case left, right {
    // leading commas are a side-effect of this cheapo parsing -- ignore them
    [",", ..left_rest], _right -> compare_packets(left_rest, right)
    _left, [",", ..right_rest] -> compare_packets(left, right_rest)

    // both sides opening -- keep going
    ["[", ..left_rest], ["[", ..right_rest] ->
      compare_packets(left_rest, right_rest)

    // both sides closing -- keep going
    ["]", ..left_rest], ["]", ..right_rest] ->
      compare_packets(left_rest, right_rest)

    // if left closes first, order is correct
    ["]", ..], _right -> order.Lt
    [], _right -> order.Lt

    // if right closes first, order is not correct
    _left, ["]", ..] -> order.Gt
    _left, [] -> order.Gt

    // first term on the right needs wrapping
    ["[", ..], _right -> {
      assert #(first_term, right_rest) =
        list.split_while(right, fn(c) { c != "," && c != "]" })
      let new_right = list.append(["[", ..first_term], ["]", ..right_rest])
      compare_packets(left, new_right)
    }

    // first term on the left needs wrapping
    _left, ["[", ..] -> {
      assert #(first_term, left_rest) =
        list.split_while(left, fn(c) { c != "," && c != "]" })
      let new_left = list.append(["[", ..first_term], ["]", ..left_rest])
      compare_packets(new_left, right)
    }

    // finally, compare two ints
    _left, _right -> {
      assert #(left_chars, left_rest) =
        list.split_while(left, fn(c) { c != "," && c != "]" })
      assert #(right_chars, right_rest) =
        list.split_while(right, fn(c) { c != "," && c != "]" })
      assert Ok(left_int) =
        left_chars
        |> string.join("")
        |> int.parse
      assert Ok(right_int) =
        right_chars
        |> string.join("")
        |> int.parse
      case int.compare(left_int, right_int) {
        order.Lt -> order.Lt
        order.Gt -> order.Gt
        order.Eq -> compare_packets(left_rest, right_rest)
      }
    }
  }
}

pub fn part1() {
  input()
  |> list.index_map(fn(i, lr) { #(i + 1, correct_order(lr)) })
  |> list.filter_map(fn(pair) {
    assert #(i, correct) = pair
    case correct {
      True -> Ok(i)
      False -> Error(Nil)
    }
  })
  |> list.fold(0, fn(acc, i) { acc + i })
  |> int.to_string
  |> io.println
}

pub fn part2() {
  let two_divider = string.to_graphemes("[[2]]")
  let six_divider = string.to_graphemes("[[6]]")

  input()
  |> list.flat_map(fn(lr) {
    assert #(left, right) = lr
    [left, right]
  })
  |> list.append([two_divider, six_divider])
  |> list.sort(compare_packets)
  |> list.index_map(fn(i, p) { #(i + 1, p) })
  |> list.filter_map(fn(pair) {
    assert #(i, p) = pair
    case p {
      p if p == two_divider -> Ok(i)
      p if p == six_divider -> Ok(i)
      _ -> Error(Nil)
    }
  })
  |> list.fold(1, fn(acc, i) { acc * i })
  |> int.to_string
  |> io.println
}
