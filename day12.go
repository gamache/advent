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
	n := 512
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

				v := countValidArrangements(parts[0], parts[1])
				v2 := countValidArrangements(
					parts[0]+"?"+parts[0],
					parts[1]+","+parts[1],
				)
				joinRatio := float32(v2) / float32(v)

				if joinRatio == float32(int32(joinRatio)) {
					// Easy way
					r := int(joinRatio)
					r = v * r * r * r * r
					results <- r
					continue
				}

				v3 := countValidArrangements(
					parts[0]+"?"+parts[0]+"?"+parts[0],
					parts[1]+","+parts[1]+","+parts[1],
				)
				joinRatio32 := float32(v3) / float32(v2)

				v4 := countValidArrangements(
					parts[0]+"?"+parts[0]+"?"+parts[0]+"?"+parts[0],
					parts[1]+","+parts[1]+","+parts[1]+","+parts[1],
				)
				joinRatio43 := float32(v4) / float32(v3)
				v5 := countValidArrangements(
					parts[0]+"?"+parts[0]+"?"+parts[0]+"?"+parts[0]+"?"+parts[0],
					parts[1]+","+parts[1]+","+parts[1]+","+parts[1]+","+parts[1],
				)
				joinRatio54 := float32(v5) / float32(v4)

				fmt.Println(line)
				fmt.Printf("%v %v %v %v %v\n", v, v2, v3, v4, v5)
				fmt.Printf("%v %v %v %v\n\n", joinRatio, joinRatio32, joinRatio43, joinRatio54)

				results <- v5
			}
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

	return cva(
		strings.Split(springMapStr+".", ""),
		pattern,
		0,
	)
}

func cva(springMap []string, pattern []int, startAt int) int {
	for i := startAt; i < len(springMap); i++ {
		if springMap[i] == "?" {
			sm1 := make([]string, len(springMap))
			copy(sm1, springMap)
			sm1[i] = "."
			va1 := 0
			if followsPatternSoFar(sm1, pattern) {
				va1 = cva(sm1, pattern, i+1)
			}

			sm2 := make([]string, len(springMap))
			copy(sm2, springMap)
			sm2[i] = "#"
			va2 := 0
			if followsPatternSoFar(sm2, pattern) {
				va2 = cva(sm2, pattern, i+1)
			}

			return va1 + va2
		}
	}

	if followsPattern(springMap, pattern) {
		return 1
	} else {
		return 0
	}
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
