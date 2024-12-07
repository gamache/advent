package main

import (
	// "container/heap"
	"fmt"
	"sort"
	"strconv"
)

func Day24() {
	lines := GetLines("inputs/day24.txt")

	numbers := make([]int, 0, len(lines))
	sum := 0

	for _, line := range lines {
		number, err := strconv.Atoi(line)
		Check(err)
		numbers = append(numbers, number)
		sum += number
	}

	type searchState struct {
		nums []int
		sum  int
	}

	//sort.Reverse(sort.IntSlice(numbers))

	for _, divisor := range []int{3, 4} {

		target := sum / divisor

		// minimum size of successsful combination
		minsize := len(numbers)

		combos := []searchState{}
		states := []searchState{searchState{[]int{}, 0}}

		for _, n := range numbers {
			newstates := []searchState{}
			for _, state := range states {
				if state.sum == target {
					if len(state.nums) <= minsize {
						minsize = len(state.nums)
						combos = append(combos, state)
					}
				} else if len(state.nums) < minsize {
					newstates = append(newstates, state)
					newnums := []int{n}
					newnums = append(newnums, state.nums...)
					newstates = append(newstates, searchState{
						nums: newnums,
						sum:  state.sum + n,
					})
				}
			}
			states = newstates
		}
		for _, state := range states {
			if state.sum == target {
				if len(state.nums) <= minsize {
					minsize = len(state.nums)
					combos = append(combos, state)
				}
			}
		}

		winstates := []searchState{}
		for _, state := range combos {
			if len(state.nums) == minsize {
				winstates = append(winstates, state)
			}
		}

		qes := []int{}
		for _, state := range winstates {
			qe := 1
			for _, n := range state.nums {
				qe *= n
			}
			qes = append(qes, qe)
		}
		sort.Sort(sort.IntSlice(qes))
		fmt.Println(qes[0])
	}
}
