package main

import (
	"fmt"
	"regexp"
	"strings"
)

func Day04() {
	lines := GetLines("inputs/day04.txt")
	day04part1(lines)
	day04part2(lines)
}

func day04part1(lines []string) {
	numberRe := regexp.MustCompile(`\d+`)
	total := 0

	for _, line := range lines {
		parts := strings.Split(line, "|")

		// the first number is the card id, throw it out
		winners := numberRe.FindAllString(parts[0], -1)[1:]
		numbers := numberRe.FindAllString(parts[1], -1)

		points := 0
	Numbers:
		for _, number := range numbers {
			for _, winner := range winners {
				if winner == number {
					if points == 0 {
						points = 1
					} else {
						points *= 2
					}
					continue Numbers
				}
			}
		}

		total += points
	}

	fmt.Println(total)
}

func day04part2(lines []string) {
	numberRe := regexp.MustCompile(`\d+`)
	matchesPerCard := []int{}
	cardCounts := []int{}

	for _, line := range lines {
		parts := strings.Split(line, "|")
		winners := numberRe.FindAllString(parts[0], -1)[1:]
		numbers := numberRe.FindAllString(parts[1], -1)
		matches := 0

	Numbers:
		for _, number := range numbers {
			for _, winner := range winners {
				if winner == number {
					matches++
					continue Numbers
				}
			}
		}

		matchesPerCard = append(matchesPerCard, matches)
		cardCounts = append(cardCounts, 1)
	}

	for i := 0; i < len(matchesPerCard); i++ {
		matches := matchesPerCard[i]
		for ii := i + 1; ii < i+1+matches; ii++ {
			cardCounts[ii] += cardCounts[i]
		}
	}

	total := 0
	for _, count := range cardCounts {
		total += count
	}

	fmt.Println(total)
}
