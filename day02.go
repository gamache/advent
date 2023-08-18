package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Day02() {
	lines := GetLines("inputs/day02.txt")

	total := 0
	ribbon := 0

	for i := 0; i < len(lines); i++ {
		line := lines[i]
		numbers := strings.Split(line, "x")
		x, err := strconv.Atoi(numbers[0])
		Check(err)
		y, err := strconv.Atoi(numbers[1])
		Check(err)
		z, err := strconv.Atoi(numbers[2])
		Check(err)

		xy := x * y
		xz := x * z
		yz := y * z

		surface_area := 2 * (xy + xz + yz)
		smallest_side := xy
		if xz < smallest_side {
			smallest_side = xz
		}
		if yz < smallest_side {
			smallest_side = yz
		}
		total += surface_area + smallest_side

		px := 2 * (y + z)
		py := 2 * (x + z)
		pz := 2 * (x + y)
		smallest_perimeter := px
		if py < smallest_perimeter {
			smallest_perimeter = py
		}
		if pz < smallest_perimeter {
			smallest_perimeter = pz
		}

		ribbon += smallest_perimeter + (x * y * z)
	}

	fmt.Println(total)
	fmt.Println(ribbon)
}
