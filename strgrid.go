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

func (sg StrGrid) Print() {
	for row := 0; row <= sg.RowMax; row++ {
		for col := 0; col <= sg.ColMax; col++ {
			c, ok := sg.Grid[Coord{row, col}]
			if ok {
				fmt.Print(c)
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println("")
	}
	fmt.Println("")
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
