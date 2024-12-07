package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day02() {
	lines := GetLines("inputs/day02.txt")
	Day02Part1(lines)
	Day02Part2(lines)
}

func Day02Part1(lines []string) {
	redMax := 12
	greenMax := 13
	blueMax := 14
	gameIdRe := regexp.MustCompile(`Game (\d+)`)
	countRe := regexp.MustCompile(`(\d+) (red|green|blue)`)
	sum := 0

Lines:
	for _, line := range lines {
		gameId, err := strconv.Atoi(gameIdRe.FindStringSubmatch(line)[1])
		Check(err)

		for _, countMatch := range countRe.FindAllStringSubmatch(line, -1) {
			count, err := strconv.Atoi(countMatch[1])
			Check(err)

			switch countMatch[2] {
			case "red":
				if count > redMax {
					continue Lines
				}
			case "green":
				if count > greenMax {
					continue Lines
				}
			case "blue":
				if count > blueMax {
					continue Lines
				}
			}
		}

		sum += gameId
	}

	fmt.Println(sum)
}

func Day02Part2(lines []string) {
	countRe := regexp.MustCompile(`(\d+) (red|green|blue)`)
	sum := 0

	for _, line := range lines {
		redMin := 0
		greenMin := 0
		blueMin := 0

		for _, countMatch := range countRe.FindAllStringSubmatch(line, -1) {
			count, err := strconv.Atoi(countMatch[1])
			Check(err)

			switch countMatch[2] {
			case "red":
				if count > redMin {
					redMin = count
				}
			case "green":
				if count > greenMin {
					greenMin = count
				}
			case "blue":
				if count > blueMin {
					blueMin = count
				}
			}
		}

		power := redMin * greenMin * blueMin
		sum += power
	}

	fmt.Println(sum)
}
