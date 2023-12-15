package main

import (
	"fmt"
	"strconv"
	"strings"
	"sync"
)

func Day12() {
	lines := GetLines("inputs/day12.txt")
	// lines := GetLines("inputs/example12.txt")

	sum := 0
	for _, line := range lines {
		parts := strings.Split(line, " ")
		v := countValidArrangements(parts[0], parts[1])
		sum += v
	}
	fmt.Println(sum)

	// part 2

	wg := sync.WaitGroup{}
	n := 1024
	work := make(chan string, n)
	results := make(chan int, n)
	for i := 0; i < n; i++ {
		go func() {
			wg.Add(1)
			defer wg.Done()
			for {
				line, ok := <-work
				if !ok {
					break
				}
				parts := strings.Split(line, " ")

				if strings.HasPrefix(parts[0], "#") || strings.HasSuffix(parts[0], "#") {
					v := countValidArrangements(parts[0], parts[1])
					results <- v * v * v * v * v
				} else {
					v5 := countValidArrangements(
						parts[0]+"?"+parts[0]+"?"+parts[0]+"?"+parts[0]+"?"+parts[0],
						parts[1]+","+parts[1]+","+parts[1]+","+parts[1]+","+parts[1],
					)
					results <- v5
				}
			}
			// }
		}()
	}

	go func() {
		wg.Add(1)
		defer wg.Done()
		for _, line := range lines {
			work <- line
		}
		close(work)
	}()

	sum = 0
	for i := range lines {
		fmt.Println(i)
		sum += <-results
	}

	// 630375483449 is too low
	// 888491519337 is too high
	// 1381452962534: also too high
	// 28126144534 is low lol
	fmt.Println(sum)

	wg.Wait()
}

func countValidArrangements(springMapStr, patternStr string) int {
	patternStrs := strings.Split(patternStr, ",")
	pattern := make([]int, len(patternStrs))

	for i, str := range patternStrs {
		p, err := strconv.Atoi(str)
		Check(err)
		pattern[i] = p
	}

	springMap := strings.Split(springMapStr+".", "")
	states := map[string]int{}

	var cva func(int) int
	cva = func(startAt int) int {
		last := "X"
		if startAt > 0 {
			last = springMap[startAt-1]
		}
		for i := startAt; i < len(springMap); i++ {
			if springMap[i] == "?" {
				springMap[i] = "."
				va1 := 0
				if followsPatternSoFar(springMap, pattern) {
					va1 = cva(i + 1)
				}

				springMap[i] = "#"
				va2 := 0
				if followsPatternSoFar(springMap, pattern) {
					va2 = cva(i + 1)
				}

				springMap[i] = "?"

				return va1 + va2
			} else if springMap[i] == "#" && last == "." {
				state := strings.Join(springMap[i+1:], "")
				va, ok := states[state]
				if ok {
					return va
				} else {
					states[state] = cva(i + 1)
					return states[state]
				}
			}
			last = springMap[i]
		}

		if followsPattern(springMap, pattern) {
			return 1
		} else {
			return 0
		}
	}
	return cva(0)
}
func followsPattern(springMap []string, pattern []int) bool {
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
