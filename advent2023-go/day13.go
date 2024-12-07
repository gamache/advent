package main

import (
	"fmt"
	"os"
	"strings"
)

func Day13() {
	data, err := os.ReadFile("inputs/day13.txt")
	Check(err)
	chunks := strings.Split(strings.TrimSpace(string(data)), "\n\n")

	sum := 0
	sum2 := 0
	for _, chunk := range chunks {
		sum += summary(chunk, 0)
		sum2 += summary(chunk, 1)
	}
	fmt.Println(sum)
	fmt.Println(sum2)
}

func summary(chunk string, smudges int) int {
	sg := FromString(chunk)

	rowsAbove := 0
	colsLeft := 0

RowsAbove:
	for row := 1; row <= sg.RowMax; row++ {
		mismatches := 0
		for dr := 1; dr < sg.RowMax; dr++ {
			for col := 0; col <= sg.ColMax; col++ {
				above, aboveOk := sg.Grid[Coord{row - dr, col}]
				below, belowOk := sg.Grid[Coord{row + dr - 1, col}]
				if aboveOk && belowOk && above != below {
					mismatches++
				}
			}
		}
		if mismatches == smudges {
			rowsAbove = row
			break RowsAbove
		}
	}

ColsLeft:
	for col := 1; col <= sg.ColMax; col++ {
		mismatches := 0
		for dc := 1; dc < sg.ColMax; dc++ {
			for row := 0; row <= sg.RowMax; row++ {
				left, leftOk := sg.Grid[Coord{row, col - dc}]
				right, rightOk := sg.Grid[Coord{row, col + dc - 1}]
				if leftOk && rightOk && left != right {
					mismatches++
				}
			}
		}
		if mismatches == smudges {
			colsLeft = col
			break ColsLeft
		}
	}

	return 100*rowsAbove + colsLeft
}
