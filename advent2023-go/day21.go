package main

import "fmt"

func Day21() {
	sg := FromLines(GetLines("inputs/day21.txt"))
	sCoord := Coord{}
	for crd, str := range sg.Grid {
		if str == "S" {
			sCoord = crd
		}
	}
	// fmt.Println(sCoord)

	coords := map[Coord]bool{sCoord: true}

	for i := 0; i < 64; i++ {
		newCoords := map[Coord]bool{}
		for crd := range coords {
			if crd.Row > 0 {
				newC := Coord{crd.Row - 1, crd.Col}
				if sg.Grid[newC] != "#" {
					newCoords[newC] = true
				}
			}
			if crd.Row < sg.RowMax {
				newC := Coord{crd.Row + 1, crd.Col}
				if sg.Grid[newC] != "#" {
					newCoords[newC] = true
				}
			}
			if crd.Col > 0 {
				newC := Coord{crd.Row, crd.Col - 1}
				if sg.Grid[newC] != "#" {
					newCoords[newC] = true
				}
			}
			if crd.Col < sg.ColMax {
				newC := Coord{crd.Row, crd.Col + 1}
				if sg.Grid[newC] != "#" {
					newCoords[newC] = true
				}
			}
		}
		coords = newCoords
	}

	fmt.Println(len(coords))

	// part 2
	prevCoords := map[Coord]bool{}
	coords = map[Coord]bool{sCoord: true}
	coordCount := 0
	for i := 0; i < 26501365; i++ {
		newCoords := map[Coord]bool{}
		if i%1000 == 0 {
			fmt.Printf("%v\t%v\t%v\n", i, len(coords), coordCount)
		}
		for crd := range coords {
			newC := Coord{(sg.RowMax + 1 + crd.Row - 1) % (sg.RowMax + 1), crd.Col}
			if sg.Grid[newC] != "#" {
				newCoords[Coord{crd.Row - 1, crd.Col}] = true
			}
			newC = Coord{(crd.Row + 1) % (sg.RowMax + 1), crd.Col}
			if sg.Grid[newC] != "#" {
				newCoords[Coord{crd.Row + 1, crd.Col}] = true
			}
			newC = Coord{crd.Row, (sg.ColMax + 1 + crd.Col - 1) % (sg.ColMax + 1)}
			if sg.Grid[newC] != "#" {
				newCoords[Coord{crd.Row, crd.Col - 1}] = true
			}
			newC = Coord{crd.Row, (crd.Col + 1) % (sg.ColMax + 1)}
			if sg.Grid[newC] != "#" {
				newCoords[Coord{crd.Row, crd.Col + 1}] = true
			}
		}
		delete(newCoords, sCoord)

		// if a coord was reachable last iteration, and it's reachable this
		// iteration, it will be reachable forever -- cull it and count it
		for crd := range prevCoords {
			if newCoords[crd] {
				coordCount++
				delete(newCoords, crd)
			}
		}

		prevCoords = coords
		coords = newCoords
	}

	fmt.Println(coordCount + len(coords))
}
