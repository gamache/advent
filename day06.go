package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day06() {
	part1()
	part2()
}

var re = regexp.MustCompile(
	`(?P<command>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)`,
)

func part1() {
	lines := GetLines("inputs/day06.txt")
	grid := make(map[Coord]bool)
	lights := 0

	for i := 0; i < len(lines); i++ {
		match := re.FindStringSubmatch(lines[i])

		// this is ridiculous for 2023
		result := make(map[string]string)
		for i, name := range re.SubexpNames() {
			if i != 0 && name != "" {
				result[name] = match[i]
			}
		}

		x1, err := strconv.Atoi(result["x1"])
		Check(err)
		y1, err := strconv.Atoi(result["y1"])
		Check(err)
		x2, err := strconv.Atoi(result["x2"])
		Check(err)
		y2, err := strconv.Atoi(result["y2"])
		Check(err)

		for x := x1; x <= x2; x++ {
			for y := y1; y <= y2; y++ {
				coord := Coord{x, y}
				switch result["command"] {
				case "turn on":
					grid[coord] = true
				case "turn off":
					grid[coord] = false
				case "toggle":
					if grid[coord] {
						grid[coord] = false
					} else {
						grid[coord] = true
					}
				default:
					panic(result["command"])
				}
			}
		}
	}

	for x := 0; x < 1000; x++ {
		for y := 0; y < 1000; y++ {
			if grid[Coord{x, y}] {
				lights++
			}
		}
	}

	fmt.Println(lights)
}

func part2() {
	lines := GetLines("inputs/day06.txt")
	grid := make(map[Coord]int)
	total_brightness := 0

	for i := 0; i < len(lines); i++ {
		match := re.FindStringSubmatch(lines[i])

		// this is ridiculous for 2023
		result := make(map[string]string)
		for i, name := range re.SubexpNames() {
			if i != 0 && name != "" {
				result[name] = match[i]
			}
		}

		x1, err := strconv.Atoi(result["x1"])
		Check(err)
		y1, err := strconv.Atoi(result["y1"])
		Check(err)
		x2, err := strconv.Atoi(result["x2"])
		Check(err)
		y2, err := strconv.Atoi(result["y2"])
		Check(err)

		for x := x1; x <= x2; x++ {
			for y := y1; y <= y2; y++ {
				coord := Coord{x, y}
				switch result["command"] {
				case "turn on":
					grid[coord] += 1
				case "turn off":
					grid[coord] -= 1
					if grid[coord] < 0 {
						grid[coord] = 0
					}
				case "toggle":
					grid[coord] += 2
				default:
					panic(result["command"])
				}
			}
		}
	}

	for x := 0; x < 1000; x++ {
		for y := 0; y < 1000; y++ {
			total_brightness += grid[Coord{x, y}]
		}
	}

	fmt.Println(total_brightness)
}
