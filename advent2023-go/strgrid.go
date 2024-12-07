package main

import (
	"fmt"
	"strings"
)

type Coord struct {
	Row int
	Col int
}

type StrGrid struct {
	RowMax int
	ColMax int
	Grid   map[Coord]string
}

func (sg StrGrid) ToString() string {
	output := ""
	for row := 0; row <= sg.RowMax; row++ {
		for col := 0; col <= sg.ColMax; col++ {
			str, ok := sg.Grid[Coord{row, col}]
			if ok {
				output += str
			} else {
				output += "."
			}
		}
		output += "\n"
	}
	return output
}

func (sg StrGrid) Print() {
	fmt.Println(sg.ToString())
}

func (sg StrGrid) Clockwise() StrGrid {
	newGrid := make(map[Coord]string, len(sg.Grid))
	for crd, str := range sg.Grid {
		newRow := crd.Col
		newCol := sg.RowMax - crd.Row
		newGrid[Coord{newRow, newCol}] = str
	}
	return StrGrid{
		RowMax: sg.ColMax,
		ColMax: sg.RowMax,
		Grid:   newGrid,
	}
}

func FromString(str string) StrGrid {
	return FromLines(strings.Split(strings.TrimSpace(str), "\n"))
}

func FromLines(lines []string) StrGrid {
	sg := StrGrid{0, 0, map[Coord]string{}}
	for row, line := range lines {
		for col, str := range strings.Split(line, "") {
			sg.Grid[Coord{row, col}] = str
			if sg.RowMax < row {
				sg.RowMax = row
			}
			if sg.ColMax < col {
				sg.ColMax = col
			}
		}
	}
	return sg
}