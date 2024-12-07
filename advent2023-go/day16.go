package main

import (
	"fmt"
)

func Day16() {
	type Beam struct {
		row  int
		col  int
		drow int
		dcol int
	}
	beamState := func(b Beam) string {
		return fmt.Sprintf("%v %v %v %v", b.row, b.col, b.drow, b.dcol)
	}

	sg := FromLines(GetLines("inputs/day16.txt"))

	energize := func(sg StrGrid, b Beam) StrGrid {
		beams := []Beam{b}
		energized := StrGrid{sg.RowMax, sg.ColMax, map[Coord]string{}}
		prevStates := map[string]bool{}

		for {
			newBeams := []Beam{}
			for _, b := range beams {
				b.row += b.drow
				b.col += b.dcol
				state := beamState(b)
				if prevStates[state] || b.row < 0 || b.row > sg.RowMax || b.col < 0 || b.col > sg.ColMax {
					continue
				}
				prevStates[state] = true
				energized.Grid[Coord{b.row, b.col}] = "#"

				switch sg.Grid[Coord{b.row, b.col}] {
				case "|":
					if b.dcol == 0 {
						newBeams = append(newBeams, b)
					} else {
						newBeams = append(newBeams, Beam{b.row, b.col, -1, 0}, Beam{b.row, b.col, 1, 0})
					}
				case "-":
					if b.dcol == 0 {
						newBeams = append(newBeams, Beam{b.row, b.col, 0, -1}, Beam{b.row, b.col, 0, 1})
					} else {
						newBeams = append(newBeams, b)
					}
				case "/":
					if b.dcol == -1 { // left to down
						newBeams = append(newBeams, Beam{b.row, b.col, 1, 0})
					} else if b.dcol == 1 { // right to up
						newBeams = append(newBeams, Beam{b.row, b.col, -1, 0})
					} else if b.drow == -1 { // up to right
						newBeams = append(newBeams, Beam{b.row, b.col, 0, 1})
					} else if b.drow == 1 { // down to left
						newBeams = append(newBeams, Beam{b.row, b.col, 0, -1})
					}
				case "\\":
					if b.dcol == -1 { // left to up
						newBeams = append(newBeams, Beam{b.row, b.col, -1, 0})
					} else if b.dcol == 1 { // right to down
						newBeams = append(newBeams, Beam{b.row, b.col, 1, 0})
					} else if b.drow == -1 { // up to left
						newBeams = append(newBeams, Beam{b.row, b.col, 0, -1})
					} else if b.drow == 1 { // down to right
						newBeams = append(newBeams, Beam{b.row, b.col, 0, 1})
					}
				default:
					newBeams = append(newBeams, b)
				}
			}
			if len(newBeams) == 0 {
				break
			}
			beams = newBeams
			//energized.Print()
		}
		return energized
	}

	// part 1

	energized := energize(sg, Beam{0, -1, 0, 1})
	fmt.Println(len(energized.Grid))

	// part 2

	maxEnergized := 0
	for row := 0; row <= sg.RowMax; row++ {
		energized = energize(sg, Beam{row, -1, 0, 1})
		if maxEnergized < len(energized.Grid) {
			maxEnergized = len(energized.Grid)
		}
		energized = energize(sg, Beam{row, sg.ColMax + 1, 0, -1})
		if maxEnergized < len(energized.Grid) {
			maxEnergized = len(energized.Grid)
		}
	}
	for col := 0; col <= sg.ColMax; col++ {
		energized = energize(sg, Beam{-1, col, 1, 0})
		if maxEnergized < len(energized.Grid) {
			maxEnergized = len(energized.Grid)
		}
		energized = energize(sg, Beam{sg.RowMax + 1, col, -1, 0})
		if maxEnergized < len(energized.Grid) {
			maxEnergized = len(energized.Grid)
		}
	}
	fmt.Println(maxEnergized)
}
