package main

func Day11() {}

/*
//// I removed this because I didn't want to refactor after making
//// strgrid/main.go

import (
	"fmt"
	"strings"
)

type StrGrid struct {
	rowMax int
	colMax int
	grid   map[coord]string
}

func (sg StrGrid) Print() {
	for row := 0; row <= sg.rowMax; row++ {
		for col := 0; col <= sg.colMax; col++ {
			c, ok := sg.grid[coord{row, col}]
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
func (sg StrGrid) WithRowsAfter(row int, count int) StrGrid {
	// fmt.Printf("WithRowsAfter %v %v\n", row, count)
	newGrid := make(map[coord]string, len(sg.grid))
	for crd, str := range sg.grid {
		if crd.row > row {
			newGrid[coord{crd.row + count, crd.col}] = str
		} else {
			newGrid[crd] = str
		}
	}
	return StrGrid{
		sg.rowMax + count,
		sg.colMax,
		newGrid,
	}
}

func (sg StrGrid) WithColumnsAfter(col int, count int) StrGrid {
	// fmt.Printf("WithColumnsAfter %v %v\n", col, count)
	newGrid := make(map[coord]string, len(sg.grid))
	for crd, str := range sg.grid {
		if crd.col > col {
			newGrid[coord{crd.row, crd.col + count}] = str
		} else {
			newGrid[crd] = str
		}
	}
	return StrGrid{
		sg.rowMax,
		sg.colMax + count,
		newGrid,
	}
}

func (sg StrGrid) Exploded(count int) StrGrid {
	// empty row expansion
	r := 0
	for {
		if r > sg.rowMax {
			break
		}
		empty := true
		for c := 0; c <= sg.colMax; c++ {
			_, ok := sg.grid[coord{r, c}]
			if ok {
				empty = false
			}
		}
		if empty {
			sg = sg.WithRowsAfter(r, count)
			r += count
		}
		r++
	}

	// empty col expansion
	c := 0
	for {
		if c > sg.colMax {
			break
		}
		empty := true
		for r := 0; r <= sg.rowMax; r++ {
			_, ok := sg.grid[coord{r, c}]
			if ok {
				empty = false
			}
		}
		if empty {
			sg = sg.WithColumnsAfter(c, count)
			c += count
		}
		c++
	}

	return sg
}

func Day11() {
	lines := GetLines("inputs/day11.txt")
	grid := make(map[coord]string, len(lines))
	rowMax := 0
	colMax := 0

	row := 0
	for _, line := range lines {
		col := 0
		for _, str := range strings.Split(line, "") {
			if str == "#" {
				grid[coord{row, col}] = str
			}
			if colMax < col {
				colMax = col
			}
			col++
		}
		if rowMax < row {
			rowMax = row
		}
		row++
	}

	// part 1
	sg := StrGrid{rowMax, colMax, grid}
	sg = sg.Exploded(1)
	fmt.Println(sumGalaxyDistances(sg))

	// part 2
	sg = StrGrid{rowMax, colMax, grid}
	sg = sg.Exploded(999999)
	fmt.Println(sumGalaxyDistances(sg))
}

func sumGalaxyDistances(sg StrGrid) int {
	galaxies := make([]coord, len(sg.grid))
	i := 0
	for galaxy := range sg.grid {
		galaxies[i] = galaxy
		i++
	}

	sum := 0
	for a := 0; a < len(galaxies); a++ {
		ga := galaxies[a]
		for b := a + 1; b < len(galaxies); b++ {
			gb := galaxies[b]

			rows := ga.row - gb.row
			if rows < 0 {
				rows = -rows
			}
			cols := ga.col - gb.col
			if cols < 0 {
				cols = -cols
			}
			distance := rows + cols
			sum += distance
		}
	}

	return sum
}
*/
