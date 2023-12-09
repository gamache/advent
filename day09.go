package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Day09() {
	lines := GetLines("inputs/day09.txt")
	day09part1(lines)
	day09part2(lines)
}

func day09part1(lines []string) {
	sum := 0

	for _, line := range lines {
		numStrs := strings.Split(line, " ")
		nums := make([]int, len(numStrs)+1)
		for i, numStr := range numStrs {
			num, err := strconv.Atoi(numStr)
			Check(err)
			nums[i] = num
		}

		matrix := [][]int{nums}

	Diffs:
		for {
			prev := matrix[len(matrix)-1]
			diffs := make([]int, len(prev)-1)
			for i := 0; i < len(diffs)-1; i++ {
				diffs[i] = prev[i+1] - prev[i]
			}
			matrix = append(matrix, diffs)
			allZeroes := true
			for _, diff := range diffs {
				if diff != 0 {
					allZeroes = false
				}
			}
			if allZeroes {
				break Diffs
			}
		}

		for i := len(matrix) - 1; i > 0; i-- {
			j := len(matrix[i])
			matrix[i-1][j] = matrix[i-1][j-1] + matrix[i][j-1]
		}

		extrapolated := matrix[0][len(matrix[0])-1]
		sum += extrapolated
	}

	fmt.Println(sum)
}

func day09part2(lines []string) {
	sum := 0

	for _, line := range lines {
		numStrs := strings.Split(line, " ")
		nums := make([]int, len(numStrs)+1)
		for i, numStr := range numStrs {
			num, err := strconv.Atoi(numStr)
			Check(err)
			nums[i+1] = num
		}

		matrix := [][]int{nums}

	Diffs:
		for {
			prev := matrix[len(matrix)-1]
			diffs := make([]int, len(prev)-1)
			for i := 1; i < len(diffs); i++ {
				diffs[i] = prev[i+1] - prev[i]
			}
			matrix = append(matrix, diffs)
			allZeroes := true
			for _, diff := range diffs {
				if diff != 0 {
					allZeroes = false
				}
			}
			if allZeroes {
				break Diffs
			}
		}

		for i := len(matrix) - 1; i > 0; i-- {
			matrix[i-1][0] = matrix[i-1][1] - matrix[i][0]
		}

		extrapolated := matrix[0][0]
		sum += extrapolated
	}

	fmt.Println(sum)
}
