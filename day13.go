package main

import (
	"fmt"
	"math"
	"regexp"
	"strconv"
)

var happyRe = regexp.MustCompile(`(?P<person1>\S+) would (?P<sign>gain|lose) (?P<amount>\d+) happiness units by sitting next to (?P<person2>\S+)\.`)

func Day13() {
	lines := GetLines("inputs/day13.txt")
	happy := make(map[string]map[string]int)

	for _, line := range lines {
		result := NamedCaptures(happyRe, line)
		amount, err := strconv.Atoi(result["amount"])
		Check(err)
		if result["sign"] == "lose" {
			amount = -amount
		}
		if happy[result["person1"]] == nil {
			happy[result["person1"]] = make(map[string]int)
		}
		happy[result["person1"]][result["person2"]] = amount
	}

	people := make([]string, 0, len(happy))
	for person, _ := range happy {
		people = append(people, person)
	}
	
	happinessOf := func(people []string) int {
		sum := 0
		for i := range people {
			j := (i + 1) % len(people)
			sum += happy[people[i]][people[j]]
			sum += happy[people[j]][people[i]]
		}
		return sum
	}
	
	optimalHappiness := math.MinInt
	
	/*
		Heap's algorithm for generating permutations, via Wikipedia:
	
		procedure generate(k : integer, A : array of any):
		if k = 1 then
			output(A)
		else
			// Generate permutations with k-th unaltered
			// Initially k = length(A)
			generate(k - 1, A)
	
			// Generate permutations for k-th swapped with each k-1 initial
			for i := 0; i < k-1; i += 1 do
				// Swap choice dependent on parity of k (even or odd)
				if k is even then
					swap(A[i], A[k-1]) // zero-indexed, the k-th is at k-1
				else
					swap(A[0], A[k-1])
				end if
				generate(k - 1, A)
			end for
		end if
	*/
	
	var permutePeople func(int)
	
	permutePeople = func(k int) {
		if k == 1 { 
			happiness := happinessOf(people)
			if optimalHappiness < happiness { optimalHappiness = happiness }
		} else {
			permutePeople(k-1)
			for i:=0; i<len(people); i++ {
				if i % 2 == 0 {
					people[i], people[k-1] = people[k-1], people[i]
				} else {
					people[0], people[k-1] = people[k-1], people[0]
				}
				permutePeople(k-1)
			}
		}
	}
	
	permutePeople(len(people))
	
	fmt.Println(optimalHappiness)
	
	// part 2
	
	optimalHappiness = math.MinInt
	people = append(people, "Me")
	permutePeople(len(people))
	fmt.Println(optimalHappiness)
}
