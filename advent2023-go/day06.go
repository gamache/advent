package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day06() {
	lines := GetLines("inputs/day06.txt")
	numberRe := regexp.MustCompile(`\d+`)
	timesStrs := numberRe.FindAllString(lines[0], -1)
	distancesStrs := numberRe.FindAllString(lines[1], -1)
	times := make([]int, len(timesStrs))
	for i, str := range timesStrs {
		time, err := strconv.Atoi(str)
		Check(err)
		times[i] = time
	}
	distances := make([]int, len(distancesStrs))
	for i, str := range distancesStrs {
		distance, err := strconv.Atoi(str)
		Check(err)
		distances[i] = distance
	}

	waysToWin := make([]int, len(times))
	for i, time := range times {
		distance := distances[i]
		for t := 0; t <= time; t++ {
			d := t * (time - t)
			if d > distance {
				waysToWin[i]++
			}
		}
	}

	product := 1
	for _, ways := range waysToWin {
		product *= ways
	}

	fmt.Println(product)

	// part 2

	timeStr := ""
	for _, str := range timesStrs {
		timeStr += str
	}
	distanceStr := ""
	for _, str := range distancesStrs {
		distanceStr += str
	}

	time, err := strconv.Atoi(timeStr)
	Check(err)
	distance, err := strconv.Atoi(distanceStr)
	Check(err)

	ways := 0
	for t := 0; t < time; t++ {
		d := t * (time - t)
		if d > distance {
			ways++
		}

	}
	fmt.Println(ways)
}