package main

import (
	"fmt"
)

func Day01(lines []string) {
	line := lines[0]
	level := 0

	for i := 0; i < len(line); i++ {
		c := line[i]
		switch c {
		case '(':
			level++
		case ')':
			level--
		default:
			panic(c)
		}
	}

	fmt.Println(level)

	// part 2

	level = 0
	for i := 0; i < len(line); i++ {
		c := line[i]
		switch c {
		case '(':
			level++
		case ')':
			level--
		default:
			panic(c)
		}
		if level == -1 {
			fmt.Println(i + 1)
			return
		}
	}
}
