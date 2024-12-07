package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type coord struct {
	row int
	col int
}

type byteGrid struct {
	rowMax int
	colMax int
	grid   map[coord]string
}

func linesToByteGrid(lines []string) byteGrid {
	rowMax := 0
	colMax := 0
	grid := map[coord]string{}

	for row, line := range lines {
		if row > rowMax {
			rowMax = row
		}
		for col, str := range strings.Split(line, "") {
			if col > colMax {
				colMax = col
			}
			grid[coord{row, col}] = str
		}
	}

	return byteGrid{rowMax, colMax, grid}
}

func Day03() {
	lines := GetLines("inputs/day03.txt")
	grid := linesToByteGrid(lines)
	day03Part1(lines, grid)
	day03Part2(lines, grid)
}

func day03Part1(lines []string, grid byteGrid) {
	sum := 0
	numberRe := regexp.MustCompile(`\d+`)

	for row, line := range lines {
		for _, indices := range numberRe.FindAllStringIndex(line, -1) {
			startCol := indices[0]
			endCol := indices[1] - 1
			number, err := strconv.Atoi(line[startCol : endCol+1])
			Check(err)

			// enumerate the coordinates surrounding the number
			coords := []coord{}
			coords = append(coords, coord{row, startCol - 1})
			coords = append(coords, coord{row, endCol + 1})
			for col := startCol - 1; col <= endCol+1; col++ {
				coords = append(coords, coord{row - 1, col})
				coords = append(coords, coord{row + 1, col})
			}

			// check for symbols
		Coords:
			for _, crd := range coords {
				str, found := grid.grid[crd]
				if !found {
					continue Coords
				}
				switch str {
				case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ".":
					continue Coords
				default:
					// this is a part number
					sum += number
					break Coords
				}
			}
		}
	}

	fmt.Println(sum)
}

type partNumber struct {
	number   int
	row      int
	startCol int
	endCol   int
}

func day03Part2(lines []string, grid byteGrid) {
	sum := 0
	numberRe := regexp.MustCompile(`\d+`)
	partNumbers := []partNumber{}

	for row, line := range lines {
		for _, indices := range numberRe.FindAllStringIndex(line, -1) {
			startCol := indices[0]
			endCol := indices[1] - 1
			number, err := strconv.Atoi(line[startCol : endCol+1])
			Check(err)
			pn := partNumber{number, row, startCol, endCol}
			partNumbers = append(partNumbers, pn)

		}
	}

	for row := 0; row <= grid.rowMax; row++ {
		for col := 0; col <= grid.colMax; col++ {
			if grid.grid[coord{row, col}] == "*" {
				adjacents := []partNumber{}
				for _, pn := range partNumbers {
					if (row == pn.row && col == pn.endCol+1) ||
						(row == pn.row && col == pn.startCol-1) ||
						(row == pn.row-1 && col >= pn.startCol-1 && col <= pn.endCol+1) ||
						(row == pn.row+1 && col >= pn.startCol-1 && col <= pn.endCol+1) {
						adjacents = append(adjacents, pn)
					}
				}
				if len(adjacents) == 2 {
					ratio := adjacents[0].number * adjacents[1].number
					sum += ratio
				}
			}
		}
	}

	fmt.Println(sum)
}
