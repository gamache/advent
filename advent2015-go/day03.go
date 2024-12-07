package main

import (
	"fmt"
)

func Day03() {
	lines := GetLines("inputs/day03.txt")

	grid := make(map[Coord]bool)
	grid[Coord{x: 0, y: 0}] = true

	for i := 0; i < len(lines); i++ {
		line := lines[i]

		x := 0
		y := 0

		for ci := 0; ci < len(line); ci++ {
			c := line[ci]
			switch c {
			case '<':
				x -= 1
			case '>':
				x += 1
			case '^':
				y += 1
			case 'v':
				y -= 1
			}
			grid[Coord{x: x, y: y}] = true
		}
	}

	fmt.Println(len(grid))

	// part 2

	grid = make(map[Coord]bool)
	grid[Coord{x: 0, y: 0}] = true
	for i := 0; i < len(lines); i++ {
		line := lines[i]

		evens := make([]byte, 0)
		odds := make([]byte, 0)

		for ci := 0; ci < len(line); ci++ {
			c := line[ci]
			if ci%2 == 0 {
				evens = append(evens, c)
			} else {
				odds = append(odds, c)
			}
		}

		x := 0
		y := 0
		for ci := 0; ci < len(evens); ci++ {
			c := evens[ci]
			switch c {
			case '<':
				x -= 1
			case '>':
				x += 1
			case '^':
				y += 1
			case 'v':
				y -= 1
			}
			grid[Coord{x: x, y: y}] = true
		}

		x = 0
		y = 0
		for ci := 0; ci < len(odds); ci++ {
			c := odds[ci]
			switch c {
			case '<':
				x -= 1
			case '>':
				x += 1
			case '^':
				y += 1
			case 'v':
				y -= 1
			}
			grid[Coord{x: x, y: y}] = true
		}
	}

	fmt.Println(len(grid))
}
