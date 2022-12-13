import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/result
import gleam/regex
import gleam/order

// #(list of chars, list of chars) tuples
type LeftAndRight =
  #(List(String), List(String))

fn input() -> List(LeftAndRight) {
  assert Ok(str) = file.read("inputs/day13-example.txt")

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
  io.println("")
  assert #(left, right) = lr
  let retval = do_correct_order(left, right)

  retval
  |> string.inspect
  |> io.println

  retval
}

fn do_correct_order(left, right) {
  io.print("Left:  ")
  left
  |> string.inspect
  |> io.println
  io.print("Right: ")
  right
  |> string.inspect
  |> io.println
  case left, right {
    // leading commas are a side-effect of this cheapo parsing -- ignore them
    [",", ..left_rest], _right -> do_correct_order(left_rest, right)
    _left, [",", ..right_rest] -> do_correct_order(left, right_rest)

    // both sides opening -- keep going
    ["[", ..left_rest], ["[", ..right_rest] ->
      do_correct_order(left_rest, right_rest)

    // both sides closing -- keep going
    ["]", ..left_rest], ["]", ..right_rest] ->
      do_correct_order(left_rest, right_rest)

    // if left closes first, order is correct
    ["]", ..], _right -> True
    [], _right -> True

    // if right closes first, order is not correct
    _left, ["]", ..] -> False
    _left, [] -> False

    // first term on the right needs wrapping
    ["[", ..], _right -> {
      assert #(first_term, right_rest) =
        list.split_while(right, fn(c) { c != "," && c != "]" })
      let new_right = list.append(["[", ..first_term], ["]", ..right_rest])
      do_correct_order(left, new_right)
    }

    // first term on the left needs wrapping
    _left, ["[", ..] -> {
      assert #(first_term, left_rest) =
        list.split_while(left, fn(c) { c != "," && c != "]" })
      let new_left = list.append(["[", ..first_term], ["]", ..left_rest])
      do_correct_order(new_left, right)
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
        order.Lt -> True
        order.Gt -> False
        order.Eq -> do_correct_order(left_rest, right_rest)
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
