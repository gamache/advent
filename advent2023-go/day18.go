package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day18() {
	lines := GetLines("inputs/day18.txt")
	lineRe := regexp.MustCompile(`([UDLR]) (\d+) .+([0-9a-f]{6})`)
	lagoon := map[Coord]bool{Coord{0, 0}: true}

	row := 0
	col := 0
	rowMax := 0
	rowMin := 0
	colMin := 0
	colMax := 0
	for _, line := range lines {
		submatches := lineRe.FindStringSubmatch(line)
		direction := submatches[1]
		distance, err := strconv.Atoi(submatches[2])
		Check(err)

		for i := 0; i < distance; i++ {
			switch direction {
			case "U":
				row--
			case "D":
				row++
			case "L":
				col--
			case "R":
				col++
			}
			if rowMax < row {
				rowMax = row
			}
			if rowMin > row {
				rowMin = row
			}
			if colMax < col {
				colMax = col
			}
			if colMin > col {
				colMin = col
			}
			lagoon[Coord{row, col}] = true
		}
	}

	floodFill := func(lagoon map[Coord]bool, start Coord, rowMin, rowMax, colMin, colMax int) {
		crds := []Coord{start}
		for {
			flips := 0
			newCrds := []Coord{}
			for _, crd := range crds {
				if crd.Row > rowMax || crd.Row < rowMin || crd.Col > colMax || crd.Col < colMin {
					continue
				}
				if !lagoon[crd] {
					flips++
					lagoon[crd] = true
					newCrds = append(newCrds,
						Coord{crd.Row - 1, crd.Col},
						Coord{crd.Row + 1, crd.Col},
						Coord{crd.Row, crd.Col - 1},
						Coord{crd.Row, crd.Col + 1},
					)
				}
			}
			if flips == 0 {
				break
			}
			crds = newCrds
		}
	}

	// find a point at the left edge of the loop, then flood right of it
	for crd := range lagoon {
		if crd.Col != colMin {
			continue
		}

		guess := Coord{crd.Row, crd.Col + 1}
		if lagoon[guess] {
			continue
		}
		floodFill(lagoon, guess, rowMin, rowMax, colMin, colMax)
		break
	}

	fmt.Println(len(lagoon))

	// part 2
	// hi I learned about the Shoelace Formula now

	vertices := []Coord{{0, 0}}
	boundaryPoints := 0
	for _, line := range lines {
		submatches := lineRe.FindStringSubmatch(line)
		hex := submatches[3]
		distance64, err := strconv.ParseInt(hex[:5], 16, 0)
		Check(err)
		distance := int(distance64)

		boundaryPoints += distance

		switch hex[5:] {
		case "0":
			col += distance
		case "1":
			row += distance
		case "2":
			col -= distance
		case "3":
			row -= distance
		}
		vertices = append(vertices, Coord{row, col})
	}

	doubleArea := 0
	for i := 0; i < len(vertices); i++ {
		prev := vertices[(len(vertices)+i-1)%len(vertices)]
		cur := vertices[i]
		next := vertices[(i+1)%len(vertices)]
		doubleArea += cur.Row * (prev.Col - next.Col)
	}
	area := AbsInt((doubleArea+boundaryPoints)/2) + 1
	fmt.Println(area)
}