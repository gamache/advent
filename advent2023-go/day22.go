package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func Day22() {
	type Cube struct {
		x, y, z int
	}
	type Brick struct {
		cubes []Cube
	}

	overlaps := func(b1, b2 Brick) bool {
		for _, c1 := range b1.cubes {
			for _, c2 := range b2.cubes {
				if c1.x == c2.x && c1.y == c2.y {
					return true
				}
			}
		}
		return false
	}
	zmax := func(b Brick) int {
		zMax := b.cubes[0].z
		for _, c := range b.cubes[1:] {
			if zMax < c.z {
				zMax = c.z
			}
		}
		return zMax
	}
	zmin := func(b Brick) int {
		zMin := b.cubes[0].z
		for _, c := range b.cubes[1:] {
			if zMin > c.z {
				zMin = c.z
			}
		}
		return zMin
	}
	drop := func(i int, bricks *[]Brick) bool {
		b := (*bricks)[i]
		if len(b.cubes) == 0 {
			return false
		}
		maxZ := 0
		for ii := 0; ii < i; ii++ {
			bb := (*bricks)[ii]
			if overlaps(b, bb) {
				zmb := zmax(bb)
				if maxZ < zmb {
					maxZ = zmb
				}
			}
		}
		dz := zmin(b) - maxZ - 1
		if dz == 0 {
			return false
		}

		newCubes := make([]Cube, len(b.cubes))
		for i, c := range b.cubes {
			newCubes[i] = Cube{c.x, c.y, c.z - dz}
		}
		b.cubes = newCubes
		(*bricks)[i] = b
		return true
	}
	brickFromLine := func(line string) Brick {
		parts := strings.Split(line, "~")
		startStrs := strings.Split(parts[0], ",")
		endStrs := strings.Split(parts[1], ",")
		x1, _ := strconv.Atoi(startStrs[0])
		y1, _ := strconv.Atoi(startStrs[1])
		z1, _ := strconv.Atoi(startStrs[2])
		x2, _ := strconv.Atoi(endStrs[0])
		y2, _ := strconv.Atoi(endStrs[1])
		z2, _ := strconv.Atoi(endStrs[2])
		if x1 > x2 {
			x1, x2 = x2, x1
		}
		if y1 > y2 {
			y1, y2 = y2, y1
		}
		if z1 > z2 {
			z1, z2 = z2, z1
		}
		cubes := []Cube{}
		for x := x1; x <= x2; x++ {
			for y := y1; y <= y2; y++ {
				for z := z1; z <= z2; z++ {
					cubes = append(cubes, Cube{x, y, z})
				}
			}
		}
		return Brick{cubes}
	}

	lines := GetLines("inputs/day22.txt")
	bricks := make([]Brick, len(lines))
	for i, line := range lines {
		bricks[i] = brickFromLine(line)
	}

	sort.Slice(bricks, func(i, j int) bool {
		return zmin(bricks[i]) < zmin(bricks[j])
	})

	for i := range bricks {
		drop(i, &bricks)
	}

	nukables := 0
	falls := 0
	for i := range bricks {
		thisFalls := 0
		otherBricks := make([]Brick, len(bricks))
		copy(otherBricks, bricks)
		otherBricks[i] = Brick{[]Cube{}}
		for j := range otherBricks {
			if drop(j, &otherBricks) {
				falls++
				thisFalls++
			}
		}
		if thisFalls == 0 {
			nukables++
		}
	}
	fmt.Println(nukables)
	fmt.Println(falls)
}
