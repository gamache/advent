package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Day01() {
	lines := GetLines("inputs/day01.txt")
	Day01Part1(lines)
	Day01Part2(lines)
}

func Day01Part1(lines []string) {
	sum := 0
	for _, line := range lines {
		firstNumAssigned := false
		var firstNum int
		var lastNum int

		for i := 0; i < len(line); i++ {
			c := line[i]
			if c >= '0' && c <= '9' {
				value, err := strconv.Atoi(string(c))
				Check(err)
				if !firstNumAssigned {
					firstNum = value
					firstNumAssigned = true
				}
				lastNum = value
			}
		}

		sum += firstNum*10 + lastNum
	}

	fmt.Println(sum)
}

func Day01Part2(lines []string) {
	digitMap := map[string]int{
		// "zero":  0,
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}

	sum := 0
	for _, line := range lines {
		firstNumAssigned := false
		var firstNum int
		var lastNum int

		for i := 0; i < len(line); i++ {
			for word, value := range digitMap {
				if strings.Index(line[i:], word) == 0 {
					if !firstNumAssigned {
						firstNum = value
						firstNumAssigned = true
					}
					lastNum = value
				}
			}

			c := line[i]
			if c >= '0' && c <= '9' {
				value, err := strconv.Atoi(string(c))
				Check(err)
				if !firstNumAssigned {
					firstNum = value
					firstNumAssigned = true
				}
				lastNum = value
			}
		}

		sum += firstNum*10 + lastNum
	}

	fmt.Println(sum)
}
