import gleam/erlang/file
import gleam/io
import gleam/string
import gleam/list
import gleam/int
import gleam/map
import gleam/result
import gleam/regex
import gleam/option

type Monkey {
  Monkey(
    name: String,
    items: List(Int),
    operation: fn(Int) -> Int,
    test: Int,
    true_monkey: String,
    false_monkey: String,
    items_handled: Int,
  )
}

fn print_monkey(monkey: Monkey) {
  io.println("Monkey " <> monkey.name)
  io.println("Items handled: " <> int.to_string(monkey.items_handled))
  monkey.items
  |> list.map(int.to_string)
  |> string.join(", ")
  |> io.println
}

type MonkeyMachine {
  MonkeyMachine(
    // Monkeys, indexed by name
    monkeys: map.Map(String, Monkey),
    // complete rounds played so far
    rounds: Int,
  )
}

fn input() -> MonkeyMachine {
  assert Ok(str) = file.read("inputs/day11.txt")

  assert Ok(name_re) = regex.from_string("Monkey (\\d+):")
  assert Ok(items_re) = regex.from_string("Starting items: (.+)")
  assert Ok(op_re) = regex.from_string("Operation: new = old (.+)")
  assert Ok(test_re) = regex.from_string("Test: divisible by (\\d+)")
  assert Ok(true_re) = regex.from_string("If true: throw to monkey (.+)")
  assert Ok(false_re) = regex.from_string("If false: throw to monkey (.+)")

  let monkeys =
    str
    |> string.trim
    |> string.split("\n\n")
    |> list.map(fn(spec_str) {
      assert [name_line, items_line, op_line, test_line, true_line, false_line] =
        spec_str
        |> string.split("\n")
        |> list.map(string.trim)

      assert [regex.Match(content: _, submatches: [option.Some(name)])] =
        regex.scan(name_re, name_line)

      assert [regex.Match(content: _, submatches: [option.Some(items_str)])] =
        regex.scan(items_re, items_line)
      let items =
        items_str
        |> string.split(", ")
        |> list.map(int.parse)
        |> result.values

      assert [regex.Match(content: _, submatches: [option.Some(op_str)])] =
        regex.scan(op_re, op_line)
      assert [operator, operand_str] = string.split(op_str, " ")
      let operation = case int.parse(operand_str) {
        Ok(operand) ->
          case operator {
            "+" -> fn(x) { x + operand }
            "-" -> fn(x) { x - operand }
            "*" -> fn(x) { x * operand }
            "/" -> fn(x) { x / operand }
            _ -> fn(x) {
              io.println("wtf")
              x
            }
          }
        _ ->
          case operator {
            "+" -> fn(x) { x + x }
            "-" -> fn(x) { x - x }
            "*" -> fn(x) { x * x }
            "/" -> fn(x) { x / x }
            _ -> fn(x) {
              io.println("wtf")
              x
            }
          }
      }

      assert [regex.Match(content: _, submatches: [option.Some(test_str)])] =
        regex.scan(test_re, test_line)
      assert Ok(test) = int.parse(test_str)

      assert [regex.Match(content: _, submatches: [option.Some(true_monkey)])] =
        regex.scan(true_re, true_line)

      assert [regex.Match(content: _, submatches: [option.Some(false_monkey)])] =
        regex.scan(false_re, false_line)

      Monkey(
        name: name,
        items: items,
        operation: operation,
        test: test,
        true_monkey: true_monkey,
        false_monkey: false_monkey,
        items_handled: 0,
      )
    })

  let monkeys_map =
    monkeys
    |> list.map(fn(m) { #(m.name, m) })
    |> map.from_list

  MonkeyMachine(monkeys: monkeys_map, rounds: 0)
}

// OK so that takes care of the input parsing. Let's solve the damn thing

fn assign_item(mm: MonkeyMachine, recipient: String, item: Int) -> MonkeyMachine {
  assert Ok(monkey) = map.get(mm.monkeys, recipient)
  let monkey = Monkey(..monkey, items: list.append(monkey.items, [item]))
  MonkeyMachine(..mm, monkeys: map.insert(mm.monkeys, monkey.name, monkey))
}

fn run_round(mm: MonkeyMachine, divide_by_three: Bool) -> MonkeyMachine {
  let mm =
    mm.monkeys
    |> map.keys
    |> list.sort(string.compare)
    |> list.fold(
      mm,
      fn(acc, monkey_name) {
        assert Ok(monkey) = map.get(acc.monkeys, monkey_name)
        print_monkey(monkey)

        let acc =
          list.fold(
            monkey.items,
            acc,
            fn(acc2, item) {
              let item = monkey.operation(item)
              let item = case divide_by_three {
                True -> item / 3
                False -> item
              }
              case int.modulo(item, monkey.test) {
                Ok(0) -> assign_item(acc2, monkey.true_monkey, item)
                _ -> assign_item(acc2, monkey.false_monkey, item)
              }
            },
          )

        let monkey =
          Monkey(
            ..monkey,
            items: [],
            items_handled: monkey.items_handled + list.length(monkey.items),
          )
        MonkeyMachine(
          ..acc,
          monkeys: map.insert(acc.monkeys, monkey.name, monkey),
        )
      },
    )
  mm.rounds + 1
  |> int.to_string
  |> io.println()
  MonkeyMachine(..mm, rounds: mm.rounds + 1)
}

pub fn part1() {
  let mm =
    list.range(1, 20)
    |> list.fold(input(), fn(mm, _) { run_round(mm, True) })

  let [first, second] =
    mm.monkeys
    |> map.values()
    |> list.map(fn(m) { m.items_handled })
    |> list.sort(int.compare)
    |> list.reverse
    |> list.take(2)

  first * second
  |> int.to_string
  |> io.println
}

pub fn part2() {
  let mm =
    list.range(1, 10000)
    |> list.fold(input(), fn(mm, _) { run_round(mm, False) })

  let [first, second] =
    mm.monkeys
    |> map.values()
    |> list.map(fn(m) { m.items_handled })
    |> list.sort(int.compare)
    |> list.reverse
    |> list.take(2)

  first * second
  |> int.to_string
  |> io.println
}
