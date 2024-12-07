package main

import "fmt"

func Day18() {
	day18part1()
	day18part2()
}

func day18part1() {
	grid := make(map[Coord]bool)
	for y, line := range GetLines("inputs/day18.txt") {
		for x := range line {
			switch line[x] {
			case '#':
				grid[Coord{x, y}] = true
			}
		}
	}

	for i := 0; i < 100; i++ {
		newgrid := make(map[Coord]bool)

		for x := 0; x < 100; x++ {
			for y := 0; y < 100; y++ {
				neighborsLit := 0
				for xx := x - 1; xx <= x+1; xx++ {
					for yy := y - 1; yy <= y+1; yy++ {
						if xx == x && yy == y {
							continue
						}
						if grid[Coord{xx, yy}] {
							neighborsLit++
						}
					}
				}

				if grid[Coord{x, y}] {
					if neighborsLit == 2 || neighborsLit == 3 {
						newgrid[Coord{x, y}] = true
					}
				} else {
					if neighborsLit == 3 {
						newgrid[Coord{x, y}] = true
					}
				}
			}
		}

		grid = newgrid
	}

	fmt.Println(len(grid))
}

func day18part2() {
	grid := make(map[Coord]bool)
	for y, line := range GetLines("inputs/day18.txt") {
		for x := range line {
			switch line[x] {
			case '#':
				grid[Coord{x, y}] = true
			}
		}
	}

	grid[Coord{0, 0}] = true
	grid[Coord{0, 99}] = true
	grid[Coord{99, 0}] = true
	grid[Coord{99, 99}] = true

	for i := 0; i < 100; i++ {
		newgrid := make(map[Coord]bool)

		for x := 0; x < 100; x++ {
			for y := 0; y < 100; y++ {
				neighborsLit := 0
				for xx := x - 1; xx <= x+1; xx++ {
					for yy := y - 1; yy <= y+1; yy++ {
						if xx == x && yy == y {
							continue
						}
						if grid[Coord{xx, yy}] {
							neighborsLit++
						}
					}
				}

				if grid[Coord{x, y}] {
					if neighborsLit == 2 || neighborsLit == 3 {
						newgrid[Coord{x, y}] = true
					}
				} else {
					if neighborsLit == 3 {
						newgrid[Coord{x, y}] = true
					}
				}
			}
		}

		newgrid[Coord{0, 0}] = true
		newgrid[Coord{0, 99}] = true
		newgrid[Coord{99, 0}] = true
		newgrid[Coord{99, 99}] = true

		grid = newgrid
	}

	fmt.Println(len(grid))
}
