package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func Day05() {
	day05part1("inputs/day05.txt")
	day05part2("inputs/day05.txt")
}

func day05part1(filename string) {
	numberRe := regexp.MustCompile(`\d+`)
	data, err := os.ReadFile(filename)
	Check(err)

	chunks := strings.Split(strings.TrimSpace(string(data)), "\n\n")

	seedStrs := numberRe.FindAllString(chunks[0], -1)
	seeds := make([]int, len(seedStrs))
	for i, seedStr := range seedStrs {
		seed, err := strconv.Atoi(seedStr)
		Check(err)
		seeds[i] = seed
	}

	lastNumbers := seeds
	for _, mappings := range chunks[1:] {
		newNumbers := make([]int, len(lastNumbers))
		copy(newNumbers, lastNumbers)

		for _, mapping := range strings.Split(mappings, "\n")[1:] {
			nums := numberRe.FindAllString(mapping, -1)
			destStart, err := strconv.Atoi(nums[0])
			Check(err)
			srcStart, err := strconv.Atoi(nums[1])
			Check(err)
			rangeLen, err := strconv.Atoi(nums[2])
			Check(err)

			for i, num := range lastNumbers {
				if num >= srcStart && num < srcStart+rangeLen {
					newNumbers[i] = destStart - srcStart + num
				}
			}
		}

		lastNumbers = newNumbers
	}

	min := lastNumbers[0]
	for _, num := range lastNumbers[1:] {
		if min > num {
			min = num
		}
	}

	fmt.Println(min)
}

type mapRange struct {
	start int
	end   int
}

// Returns [mappedRanges], [unmappedRanges]
func (mr mapRange) applyMapping(destStart, srcStart, rangeLen int) ([]mapRange, []mapRange) {
	srcEnd := srcStart + rangeLen - 1
	mrLen := mr.end - mr.start + 1

	mappedRanges := []mapRange{}
	unmappedRanges := []mapRange{}

	if mr.end < srcStart || mr.start > srcEnd {
		// mr is entirely unmapped
		unmappedRanges = append(unmappedRanges, mr)
	} else if mr.start >= srcStart && mr.end <= srcEnd {
		// mr is entirely mapped
		mappedStart := destStart + (mr.start - srcStart)
		mappedEnd := mappedStart + mrLen - 1
		mappedRanges = append(mappedRanges, mapRange{mappedStart, mappedEnd})
	} else if mr.start >= srcStart {
		// the low side of mr is mapped
		unmappedLen := mr.end - srcEnd
		mappedStart := destStart + (mr.start - srcStart)
		mappedEnd := mappedStart + mrLen - unmappedLen
		mappedRanges = append(mappedRanges, mapRange{mappedStart, mappedEnd})
		unmappedRanges = append(unmappedRanges, mapRange{srcEnd + 1, srcEnd + unmappedLen})
	} else if mr.end <= srcEnd {
		// the high side of mr is mapped
		unmappedLen := srcStart - mr.start
		mappedLen := mrLen - unmappedLen
		mappedRanges = append(mappedRanges, mapRange{destStart, destStart + mappedLen})
		unmappedRanges = append(unmappedRanges, mapRange{mr.start, srcStart - 1})
	} else {
		// the middle of mr is mapped but the ends are not
		mappedRanges = append(mappedRanges, mapRange{destStart, destStart + rangeLen - 1})
		unmappedRanges = append(unmappedRanges,
			mapRange{mr.start, srcStart - 1},
			mapRange{srcEnd + 1, mr.end},
		)
	}

	return mappedRanges, unmappedRanges
}

func day05part2(filename string) {
	numberRe := regexp.MustCompile(`\d+`)
	data, err := os.ReadFile(filename)
	Check(err)

	chunks := strings.Split(strings.TrimSpace(string(data)), "\n\n")

	seedStrs := numberRe.FindAllString(chunks[0], -1)
	seedNums := make([]int, len(seedStrs))
	for i, seedStr := range seedStrs {
		seed, err := strconv.Atoi(seedStr)
		Check(err)
		seedNums[i] = seed
	}

	unmappedRangeSet := make(map[mapRange]bool, len(seedNums)/2)
	for i := 0; i < len(seedNums)/2; i++ {
		start := seedNums[2*i]
		length := seedNums[2*i+1]
		mr := mapRange{start, start + length - 1}
		unmappedRangeSet[mr] = true
	}

	for _, mappings := range chunks[1:] {
		mappedRangeSet := map[mapRange]bool{}

		for _, mapping := range strings.Split(mappings, "\n")[1:] {
			nums := numberRe.FindAllString(mapping, -1)
			destStart, err := strconv.Atoi(nums[0])
			Check(err)
			srcStart, err := strconv.Atoi(nums[1])
			Check(err)
			rangeLen, err := strconv.Atoi(nums[2])
			Check(err)

			for mr := range unmappedRangeSet {
				newMapped, newUnmapped := mr.applyMapping(
					destStart,
					srcStart,
					rangeLen,
				)

				if len(newMapped) > 0 {
					delete(unmappedRangeSet, mr)
				}
				for _, mr := range newMapped {
					mappedRangeSet[mr] = true
				}
				for _, mr := range newUnmapped {
					unmappedRangeSet[mr] = true
				}
			}
		}

		for mr := range mappedRangeSet {
			unmappedRangeSet[mr] = true
		}
	}

	min := -1
	for mr := range unmappedRangeSet {
		if min < 0 || min > mr.start {
			min = mr.start
		}
	}

	fmt.Println(min)
}
