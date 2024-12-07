package main

import (
	"fmt"
	"regexp"
	"strconv"
)

type ingredientSpec struct {
	capacity   int
	durability int
	flavor     int
	texture    int
	calories   int
}

func Day15() {
	ingredients := make(map[int]ingredientSpec)

	lines := GetLines("inputs/day15.txt")
	lineRe := regexp.MustCompile(`(?P<name>\S+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)`)
	for i, line := range lines {
		result := NamedCaptures(lineRe, line)

		capacity, err := strconv.Atoi(result["capacity"])
		Check(err)
		durability, err := strconv.Atoi(result["durability"])
		Check(err)
		flavor, err := strconv.Atoi(result["flavor"])
		Check(err)
		texture, err := strconv.Atoi(result["texture"])
		Check(err)
		calories, err := strconv.Atoi(result["calories"])
		Check(err)

		ingredients[i] = ingredientSpec{
			capacity:   capacity,
			durability: durability,
			flavor:     flavor,
			texture:    texture,
			calories:   calories,
		}
	}

	highScore := 0
	highScore500 := 0
	// this all gets easier when coded for the correct number of ingredients
	for a := 0; a <= 100; a++ {
		for b := 0; b <= (100 - a); b++ {
			for c := 0; c <= (100 - a - b); c++ {
				d := 100 - a - b - c

				aa := ingredients[0]
				bb := ingredients[1]
				cc := ingredients[2]
				dd := ingredients[3]

				capacity := 0
				durability := 0
				flavor := 0
				texture := 0
				calories := 0

				capacity += a * aa.capacity
				capacity += b * bb.capacity
				capacity += c * cc.capacity
				capacity += d * dd.capacity
				if capacity < 0 {
					capacity = 0
				}

				durability += a * aa.durability
				durability += b * bb.durability
				durability += c * cc.durability
				durability += d * dd.durability
				if durability < 0 {
					durability = 0
				}

				flavor += a * aa.flavor
				flavor += b * bb.flavor
				flavor += c * cc.flavor
				flavor += d * dd.flavor
				if flavor < 0 {
					flavor = 0
				}

				texture += a * aa.texture
				texture += b * bb.texture
				texture += c * cc.texture
				texture += d * dd.texture
				if texture < 0 {
					texture = 0
				}

				calories += a * aa.calories
				calories += b * bb.calories
				calories += c * cc.calories
				calories += d * dd.calories

				score := capacity * durability * flavor * texture
				if highScore < score {
					highScore = score
				}
				if calories == 500 && highScore500 < score {
					highScore500 = score
				}
			}
		}
	}

	fmt.Println(highScore)
	fmt.Println(highScore500)
}