package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day16() {
	lines := GetLines("inputs/day16.txt")
	sues := make(map[int]map[string]int)
	lineRe := regexp.MustCompile(
		`Sue (?P<sue>\d+): (?P<k1>\S+): (?P<v1>\d+), (?P<k2>\S+): (?P<v2>\d+), (?P<k3>\S+): (?P<v3>\d+)`,
	)

	for _, line := range lines {
		result := NamedCaptures(lineRe, line)
		sue, err := strconv.Atoi(result["sue"])
		Check(err)
		v1, err := strconv.Atoi(result["v1"])
		Check(err)
		v2, err := strconv.Atoi(result["v2"])
		Check(err)
		v3, err := strconv.Atoi(result["v3"])
		Check(err)

		sueSpec := make(map[string]int)
		sueSpec[result["k1"]] = v1
		sueSpec[result["k2"]] = v2
		sueSpec[result["k3"]] = v3

		sues[sue] = sueSpec
	}

	measuredSue := map[string]int{
		"children":    3,
		"cats":        7,
		"samoyeds":    2,
		"pomeranians": 3,
		"akitas":      0,
		"vizslas":     0,
		"goldfish":    5,
		"trees":       3,
		"cars":        2,
		"perfumes":    1,
	}

	sueMatches := func(sueSpec map[string]int) bool {
		for key, value := range sueSpec {
			if measuredSue[key] != value {
				return false
			}
		}
		return true
	}

	for n, sueSpec := range sues {
		if sueMatches(sueSpec) {
			fmt.Println(n)
		}
	}

	// part 2

	sueMatches2 := func(sueSpec map[string]int) bool {
		for key, value := range sueSpec {
			switch key {
			case "cats", "trees":
				if measuredSue[key] >= value {
					return false
				}
			case "pomeranians", "goldfish":
				if measuredSue[key] <= value {
					return false
				}
			default:
				if measuredSue[key] != value {
					return false
				}
			}
		}
		return true
	}

	for n, sueSpec := range sues {
		if sueMatches2(sueSpec) {
			fmt.Println(n)
		}
	}
}