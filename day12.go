package main

import (
	"fmt"
	"strconv"
	"strings"
	"sync"
)

func Day12() {
	lines := GetLines("inputs/day12.txt")

	sum := 0
	for _, line := range lines {
		fmt.Println(line)
		parts := strings.Split(line, " ")

		v := countValidArrangements(parts[0], parts[1])
		sum += v
	}
	fmt.Println(sum)

	// part 2

	wg := sync.WaitGroup{}
	work := make(chan string)
	results := make(chan int)

	for i := 0; i < 64; i++ {
		go func() {
			wg.Add(1)
			defer wg.Done()

			for {
				line, ok := <-work
				if !ok {
					break
				}
				fmt.Println(line)
				parts := strings.Split(line, " ")
				results <- countValidArrangements(
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

	sum2 := 0
	for _ = range lines {
		sum2 += <-results
	}
	fmt.Println(sum2)

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

	arrangements := cva(
		strings.Split(springMapStr+".", ""),
		pattern,
		0,
	)

	// fmt.Printf("%v %v %v\n", springMapStr, patternStr, arrangements)

	return arrangements
	/*
	   // fmt.Println(springMapStr)
	   count := 0



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
	*/
}

func cva(springMap []string, pattern []int, startAt int) int {
	// fmt.Printf("cva   %v %v %v\n", springMap, pattern, startAt)

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
		// fmt.Printf("match %v %v\n", springMap, pattern)
		return 1
	} else {
		return 0
	}
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
