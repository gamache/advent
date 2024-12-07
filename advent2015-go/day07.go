package main

import (
	"fmt"
	"regexp"
	"strconv"
)

var out_re = regexp.MustCompile(
	`-> (?P<out>\S+)$`,
)
var assignment_re = regexp.MustCompile(
	`^(?P<input>\S+) -> (?P<out>\S+)$`,
)
var andor_re = regexp.MustCompile(
	`^(?P<input1>\S+) (?P<op>AND|OR) (?P<input2>\S+) -> (?P<out>\S+)$`,
)
var shift_re = regexp.MustCompile(
	`^(?P<input>\S+) (?P<op>RSHIFT|LSHIFT) (?P<shift>\d+) -> (?P<out>\S+)$`,
)
var not_re = regexp.MustCompile(
	`^NOT (?P<input>\S+) -> (?P<out>\S+)$`,
)

func Day07() {
	lines := GetLines("inputs/day07.txt")
	wires := make(map[string]int)

	wires = solve(lines, wires)
	a := wires["a"]
	fmt.Println(a)

	wires = make(map[string]int)
	wires["b"] = a
	wires = solve(lines, wires)
	a = wires["a"]
	fmt.Println(a)
}

func solve(lines []string, wires map[string]int) map[string]int {
	for {
		if len(wires) >= len(lines) {
			break
		}

		for i := range lines {
			line := lines[i]

			result := NamedCaptures(out_re, line)
			_, present := wires[result["out"]]
			if present {
				continue
			}

			result = NamedCaptures(assignment_re, line)
			if result != nil {
				input, present := wires[result["input"]]
				literal, err := strconv.Atoi(result["input"])
				if err == nil {
					input = literal
					present = true
				}
				if present {
					wires[result["out"]] = input
				}
				continue
			}

			result = NamedCaptures(andor_re, line)
			if result != nil {
				signal1, present1 := wires[result["input1"]]
				literal1, err := strconv.Atoi(result["input1"])
				if err == nil {
					signal1 = literal1
					present1 = true
				}

				signal2, present2 := wires[result["input2"]]
				literal2, err := strconv.Atoi(result["input2"])
				if err == nil {
					signal2 = literal2
					present2 = true
				}

				if present1 && present2 {
					var signal int

					switch result["op"] {
					case "AND":
						signal = signal1 & signal2
					case "OR":
						signal = signal1 | signal2
					}

					wires[result["out"]] = signal & 0xffff
				}
				continue
			}

			result = NamedCaptures(shift_re, line)
			if result != nil {
				input, present := wires[result["input"]]
				literal, err := strconv.Atoi(result["input"])
				if err == nil {
					input = literal
					present = true
				}

				if present {
					shift, err := strconv.Atoi(result["shift"])
					Check(err)

					var signal int
					switch result["op"] {
					case "RSHIFT":
						signal = input >> shift
					case "LSHIFT":
						signal = input << shift
					}

					wires[result["out"]] = signal & 0xffff
				}
				continue
			}

			result = NamedCaptures(not_re, line)
			if result != nil {
				input, present := wires[result["input"]]
				literal, err := strconv.Atoi(result["input"])
				if err == nil {
					input = literal
					present = true
				}

				if present {
					wires[result["out"]] = (^input) & 0xffff
				}
				continue
			}

			fmt.Printf("couldn't parse line: %v\n", line)
		}
	}

	return wires
}
