package main

import (
	"fmt"
	"strings"
)

func Day10() {
	lines := GetLines("inputs/day10.txt")
	grid := make(map[coord]string, len(lines))
	rowMax := len(lines) - 1
	colMax := 0
	for row, line := range lines {
		for col, str := range strings.Split(line, "") {
			grid[coord{row, col}] = str
			if colMax < col {
				colMax = col
			}
		}
	}

	startRow := 0
	startCol := 0

FindStart:
	for row := 0; row <= rowMax; row++ {
		for col := 0; col <= colMax; col++ {
			if grid[coord{row, col}] == "S" {
				startRow = row
				startCol = col
				break FindStart
			}
		}
	}

	visited := map[coord]bool{}
	distance := 0
	row := startRow
	col := startCol

Traverse:
	for {
		visited[coord{row, col}] = true
		distance++

		switch grid[coord{row, col}] {
		case "S":
			// just pick a direction
			if grid[coord{row - 1, col}] == "|" ||
				grid[coord{row - 1, col}] == "F" ||
				grid[coord{row - 1, col}] == "7" {
				row -= 1
			} else if grid[coord{row + 1, col}] == "|" ||
				grid[coord{row + 1, col}] == "L" ||
				grid[coord{row + 1, col}] == "J" {
				row += 1
			} else if grid[coord{row, col - 1}] == "-" ||
				grid[coord{row, col - 1}] == "F" ||
				grid[coord{row, col - 1}] == "L" {
				col -= 1
			} else if grid[coord{row, col + 1}] == "-" ||
				grid[coord{row, col + 1}] == "J" ||
				grid[coord{row, col + 1}] == "7" {
				col += 1
			}

		case "-":
			if !visited[coord{row, col - 1}] {
				col -= 1
			} else if !visited[coord{row, col + 1}] {
				col += 1
			} else {
				break Traverse
			}

		case "|":
			if !visited[coord{row - 1, col}] {
				row -= 1
			} else if !visited[coord{row + 1, col}] {
				row += 1
			} else {
				break Traverse
			}

		case "F":
			if !visited[coord{row, col + 1}] {
				col += 1
			} else if !visited[coord{row + 1, col}] {
				row += 1
			} else {
				break Traverse
			}

		case "L":
			if !visited[coord{row - 1, col}] {
				row -= 1
			} else if !visited[coord{row, col + 1}] {
				col += 1
			} else {
				break Traverse
			}

		case "7":
			if !visited[coord{row, col - 1}] {
				col -= 1
			} else if !visited[coord{row + 1, col}] {
				row += 1
			} else {
				break Traverse
			}

		case "J":
			if !visited[coord{row - 1, col}] {
				row -= 1
			} else if !visited[coord{row, col - 1}] {
				col -= 1
			} else {
				break Traverse
			}
		}
	}

	fmt.Println(distance / 2)

	// part 2

	// Strategy: first draw the graph twice as big, then paintbucket
	// from outside in

	dblGrid := map[coord]string{}
	for crd := range visited {
		rr := crd.row * 2
		cc := crd.col * 2

		if !visited[crd] {
			continue
		}

		switch grid[crd] {
		case "|":
			dblGrid[coord{rr - 1, cc}] = "|"
			dblGrid[coord{rr, cc}] = "|"
			dblGrid[coord{rr + 1, cc}] = "|"
		case "-":
			dblGrid[coord{rr, cc - 1}] = "-"
			dblGrid[coord{rr, cc}] = "-"
			dblGrid[coord{rr, cc + 1}] = "-"
		case "F":
			dblGrid[coord{rr, cc + 1}] = "-"
			dblGrid[coord{rr, cc}] = "F"
			dblGrid[coord{rr + 1, cc}] = "|"
		case "L":
			dblGrid[coord{rr, cc + 1}] = "-"
			dblGrid[coord{rr, cc}] = "L"
			dblGrid[coord{rr - 1, cc}] = "|"
		case "7":
			dblGrid[coord{rr, cc - 1}] = "-"
			dblGrid[coord{rr, cc}] = "7"
			dblGrid[coord{rr + 1, cc}] = "|"
		case "J":
			dblGrid[coord{rr, cc - 1}] = "-"
			dblGrid[coord{rr, cc}] = "J"
			dblGrid[coord{rr - 1, cc}] = "|"
		case "S":
			dblGrid[coord{rr, cc}] = "S"
		}
	}

	// define outside border
	for rr := -1; rr <= (rowMax*2)+1; rr++ {
		dblGrid[coord{rr, -1}] = "O"
		dblGrid[coord{rr, colMax*2 + 1}] = "O"
	}
	for cc := -1; cc <= (colMax*2)+1; cc++ {
		dblGrid[coord{-1, cc}] = "O"
		dblGrid[coord{rowMax*2 + 1, cc}] = "O"
	}

Paintbucket:
	for {
		flips := false
		for rr := -1; rr <= rowMax*2+1; rr++ {
			for cc := -1; cc <= colMax*2+1; cc++ {
				_, ok := dblGrid[coord{rr, cc}]
				if !ok &&
					(dblGrid[coord{rr - 1, cc}] == "O" ||
						dblGrid[coord{rr + 1, cc}] == "O" ||
						dblGrid[coord{rr, cc - 1}] == "O" ||
						dblGrid[coord{rr, cc + 1}] == "O") {
					flips = true
					dblGrid[coord{rr, cc}] = "O"
				}
			}
		}

		if !flips {
			break Paintbucket
		}
	}

	enclosed := 0
	for row := 0; row <= rowMax; row++ {
		for col := 0; col <= colMax; col++ {
			_, ok := dblGrid[coord{row * 2, col * 2}]
			if !ok {
				enclosed++
			}
		}
	}
	fmt.Println(enclosed)
}
