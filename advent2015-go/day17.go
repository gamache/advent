package main

import (
	"fmt"
	"strconv"
)

func Day17() {
	lines := GetLines("inputs/day17.txt")
	sizes := make([]int, 0, len(lines))
	for _, line := range lines {
		size, err := strconv.Atoi(line)
		Check(err)
		sizes = append(sizes, size)
	}

	combinations := 0
	combosByCount := make(map[int]int)
	for i := 0; i < (1 << len(sizes)); i++ {
		total := 0
		popcount := 0
		for n, size := range sizes {
			if i&(1<<n) != 0 {
				popcount++
				total += size
			}
		}
		if total == 150 {
			combinations++
			combosByCount[popcount]++
		}
	}

	fmt.Println(combinations)
	fmt.Println(combosByCount)
}
