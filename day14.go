package main

import (
	"fmt"
	// "sort"
)

func Day14() {
	filename := "inputs/day14.txt"

	// part 1
	sg := FromLines(GetLines(filename))
	sg = rollNorth(sg)
	fmt.Println(northLoad(sg))

	// part 2
	sg = FromLines(GetLines(filename))
	previousStates := map[string]int{}
	iters := 1_000_000_000
	for i := 0; i < iters; i++ {
		prevIter, ok := previousStates[sg.ToString()]
		if ok {
			// we have detected a cycle
			period := i - prevIter
			for {
				if i+period < iters {
					i += period
				} else {
					break
				}
			}
			// forget everything we know so we can finish the loop
			previousStates = map[string]int{}
		} else {
			previousStates[sg.ToString()] = i
		}
		sg = rollNorth(sg)
		sg = sg.Clockwise()
		sg = rollNorth(sg)
		sg = sg.Clockwise()
		sg = rollNorth(sg)
		sg = sg.Clockwise()
		sg = rollNorth(sg)
		sg = sg.Clockwise()
	}

	fmt.Println(northLoad(sg))
}

func rollNorth(sg StrGrid) StrGrid {
	newGrid := make(map[Coord]string, len(sg.Grid))

	for col := 0; col <= sg.ColMax; col++ {
		stopAtRow := 0
		for row := 0; row <= sg.RowMax; row++ {
			str, ok := sg.Grid[Coord{row, col}]
			if ok {
				if str == "#" {
					newGrid[Coord{row, col}] = "#"
					stopAtRow = row + 1
				} else if str == "O" {
					newGrid[Coord{stopAtRow, col}] = "O"
					stopAtRow++
				}
			}
		}
	}

	return StrGrid{sg.RowMax, sg.ColMax, newGrid}
}

func northLoad(sg StrGrid) int {
	load := 0
	for col := 0; col <= sg.ColMax; col++ {
		for row := 0; row <= sg.RowMax; row++ {
			str := sg.Grid[Coord{row, col}]
			if str == "O" {
				d := sg.RowMax + 1 - row
				load += d
			}
		}
	}
	return load
}
