package main

import (
	"fmt"
	"regexp"
	"strconv"
)

type speed struct {
	magnitude int
	duration  int
	rest      int
}

func Day14() {
	speedRe := regexp.MustCompile(`(?P<name>\S+) can fly (?P<magnitude>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.`)

	speeds := make(map[string]speed)

	lines := GetLines("inputs/day14.txt")
	for _, line := range lines {
		result := NamedCaptures(speedRe, line)

		magnitude, err := strconv.Atoi(result["magnitude"])
		Check(err)
		duration, err := strconv.Atoi(result["duration"])
		Check(err)
		rest, err := strconv.Atoi(result["rest"])
		Check(err)

		speeds[result["name"]] = speed{
			magnitude: magnitude,
			duration:  duration,
			rest:      rest,
		}
	}

	distanceAt := func(spd speed, time int) int {
		period := spd.duration + spd.rest
		full_periods := time / period
		remainder := time % period

		total := full_periods * spd.magnitude * spd.duration
		if remainder > spd.duration {
			remainder = spd.duration
		}
		total += remainder * spd.magnitude
		return total
	}

	distances := make(map[string]int)
	for name, spd := range speeds {
		distances[name] = distanceAt(spd, 2503)
	}

	maxDistance := 0
	for _, distance := range distances {
		if maxDistance < distance {
			maxDistance = distance
		}
	}
	fmt.Println(maxDistance)

	// part 2

	scores := make(map[string]int)

	for time := 1; time <= 2503; time++ {
		winnerNames := []string{}
		winnerDistance := 0

		for name, spd := range speeds {
			distance := distanceAt(spd, time)
			if winnerDistance == distance {
				winnerNames = append(winnerNames, name)
			} else if winnerDistance < distance {
				winnerDistance = distance
				winnerNames = []string{name}
			}
		}

		for _, name := range winnerNames {
			scores[name]++
		}
	}

	maxScore := 0
	for _, score := range scores {
		if maxScore < score {
			maxScore = score
		}
	}
	fmt.Println(maxScore)
}
