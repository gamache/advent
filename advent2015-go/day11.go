package main

import (
	"fmt"
)

func Day11() {
	input := GetLines("inputs/day11.txt")[0]

	password := nextValidPassword(input)
	fmt.Println(password)
	fmt.Println(nextValidPassword(password))
}

func nextValidPassword(str string) string {
	password := str
	for {
		password = incrementPassword(password)
		if passwordIsValid(password) {
			break
		}
	}
	return password
}

func incrementPassword(str string) string {
	// add a new digit if we need one
	if len(str) == 0 {
		return "a"
	}

	lastLetter := str[len(str)-1]
	switch lastLetter {
	case 'h':
		return str[0:len(str)-1] + "j" // skip "i"
	case 'k':
		return str[0:len(str)-1] + "m" // skip "l"
	case 'n':
		return str[0:len(str)-1] + "p" // skip "o"
	case 'z':
		return incrementPassword(str[0:len(str)-1]) + "a"
	default:
		return str[0:len(str)-1] + string(lastLetter+1)
	}
}

func passwordIsValid(str string) bool {
	return runOfThree(str) && noILO(str) && twoDifferentPairs(str)
}

func runOfThree(str string) bool {
	var a, b byte
	for i := range str {
		c := str[i]
		if a+1 == b && b+1 == c {
			return true
		}
		a = b
		b = c
	}
	return false
}

func noILO(str string) bool {
	for _, rune := range str {
		if rune == 'i' || rune == 'l' || rune == 'o' {
			return false
		}
	}
	return true
}

func twoDifferentPairs(str string) bool {
	pairs := make(map[rune]int)
	var lastRune rune
	for _, rune := range str {
		if rune == lastRune {
			pairs[rune]++
		}
		lastRune = rune
	}
	return len(pairs) > 1
}
