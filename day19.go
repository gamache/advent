package main

import (
	"fmt"
	"math/rand"
	"strings"
)

func Day19() {
	lines := GetLines("inputs/day19.txt")
	origMedicine := lines[len(lines)-1]
	medicine := origMedicine

	replacements := make(map[string][]string)
	for _, line := range lines[0 : len(lines)-2] {
		repl := strings.Split(line, " => ")
		from := repl[0]
		to := repl[1]
		if replacements[from] == nil {
			replacements[from] = []string{to}
		} else {
			replacements[from] = append(replacements[from], to)
		}
	}

	uniques := make(map[string]int)
	soFar := ""

NextToken:
	for {
		if medicine == "" {
			break
		}

		for from, tos := range replacements {
			if strings.Index(medicine, from) == 0 {
				medicine = medicine[len(from):len(medicine)]
				for _, to := range tos {
					uniques[soFar+to+medicine]++
				}

				soFar += from
				continue NextToken
			}
		}

		// no match this time, advance 1 char and try again
		soFar += medicine[0:1]
		medicine = medicine[1:len(medicine)]
	}
	
	fmt.Println(len(uniques))
	
	// part 2
	
	invReplacements := make(map[string]string)
	invFroms := make([]string, 0, len(invReplacements))
	for from, tos := range replacements {
		for _, to := range tos {
				invReplacements[to] = from
		}
	}
	for from, _ := range invReplacements {
		invFroms = append(invFroms, from)
	}
	
	replacementCount := 0
	medicine = origMedicine
	
	for {
		if medicine == "e" {
			break
		}
		
		tmpMedicine := medicine
		for _, from := range invFroms {
			to := invReplacements[from]
			index := strings.Index(medicine, from)
			if index > -1 {
				replacementCount++
				medicine = strings.Replace(medicine, from, to, 1)
			}
		}
		
		// if we're wedged, reroll and try again
		if tmpMedicine == medicine {
			rand.Shuffle(len(invFroms), func(i,j int) {
				invFroms[i], invFroms[j] = invFroms[j], invFroms[i]
			})
			replacementCount = 0
			medicine = origMedicine
		}
	}
	
	fmt.Println(replacementCount)
}
