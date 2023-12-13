package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Day12() {
	lines := GetLines("inputs/day12.txt")

	// part 1
	sum := 0
	for _, line := range lines {
		parts := strings.Split(line, " ")
		v := countValidArrangements(parts[0], parts[1])
		sum += v
	}
	fmt.Println(sum)

	// part 2
	sum = 0
	for _, line := range lines {
		fmt.Println(line)
		parts := strings.Split(line, " ")
		v := countValidArrangements(
			strings.Join([]string{
				parts[0],
				parts[0],
				parts[0],
				parts[0],
				parts[0],
			}, "?"),
			strings.Join([]string{
				parts[1],
				parts[1],
				parts[1],
				parts[1],
				parts[1],
			}, ","),
		)
		sum += v
	}
	fmt.Println(sum)

}

func countValidArrangements(springMapStr, patternStr string) int {
	// fmt.Println(springMapStr)
	count := 0

	patternStrs := strings.Split(patternStr, ",")
	pattern := make([]int, len(patternStrs))
	for i, str := range patternStrs {
		p, err := strconv.Atoi(str)
		Check(err)
		pattern[i] = p
	}

	arrangements := []string{springMapStr}
	for _, str := range strings.Split(springMapStr, "") {
		if str == "?" {
			newArrangements := []string{}
			for _, arr := range arrangements {
				// fmt.Println(arr)
				newArr1 := strings.Replace(arr, "?", ".", 1)
				if followsPatternSoFar(strings.Split(newArr1, ""), pattern) {
					newArrangements = append(newArrangements, newArr1)
				}
				newArr2 := strings.Replace(arr, "?", "#", 1)
				if followsPatternSoFar(strings.Split(newArr2, ""), pattern) {
					newArrangements = append(newArrangements, newArr2)
				}
			}
			arrangements = newArrangements
		}
	}

	// fmt.Println(arrangements)

	for _, arr := range arrangements {
		springMap := strings.Split(arr, "")
		springMap = append(springMap, ".") // makes logic easier below
		if followsPattern(springMap, pattern) {
			count++
		}
	}

	return count
}

func followsPattern(springMap []string, pattern []int) bool {
	// fmt.Printf("followsPattern %v %v\n", springMap, pattern)

	count := 0
	last := "."

	for i, str := range springMap {
		switch str {
		case "#":
			count++
		case ".":
			if last == "#" {
				if len(pattern) == 0 {
					return false
				}
				if count == pattern[0] {
					return followsPattern(springMap[i+1:], pattern[1:])
				} else {
					return false
				}
			}
		}
		last = str
	}

	return len(pattern) == 0
}

func followsPatternSoFar(springMap []string, pattern []int) bool {
	// fmt.Printf("followsPatternSoFar %v %v\n", springMap, pattern)

	count := 0
	last := "."

	for i, str := range springMap {
		switch str {
		case "?":
			return true
		case "#":
			count++
			if len(pattern) == 0 {
				return false
			}
			if count > pattern[0] {
				return false
			}
		case ".":
			if last == "#" {
				if len(pattern) == 0 {
					return false
				}
				if count == pattern[0] {
					return followsPatternSoFar(springMap[i+1:], pattern[1:])
				} else {
					return false
				}
			}
		}
		last = str
	}

	return true
}
