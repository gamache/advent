package main

import (
	"fmt"
)

func Day05() {
	lines := GetLines("inputs/day05.txt")
	nice_count := 0
	nice2_count := 0
	for i := 0; i < len(lines); i++ {
		if nice(lines[i]) {
			nice_count++
		}
		if nice2(lines[i]) {
			nice2_count++
		}
	}
	fmt.Println(nice_count)
	fmt.Println(nice2_count)
}

func nice(str string) bool {
	vowels := 0
	doubles := 0

	var last_letter byte = 0
	for i := 0; i < len(str); i++ {
		c := str[i]

		switch c {
		case 'a':
			vowels++
		case 'e':
			vowels++
		case 'i':
			vowels++
		case 'o':
			vowels++
		case 'u':
			vowels++
		}

		if (last_letter == 'a' && c == 'b') ||
			(last_letter == 'c' && c == 'd') ||
			(last_letter == 'p' && c == 'q') ||
			(last_letter == 'x' && c == 'y') {
			return false
		}

		if c == last_letter {
			doubles++
		}
		last_letter = c
	}

	return vowels >= 3 && doubles >= 1
}

func nice2(str string) bool {
	repeat_with_one_letter_between := false
	repeat_with_no_overlap := false

	var last_letter byte = 0
	var second_to_last_letter byte = 0
	prev_two_letter_map := make(map[string]int)

	for i := 0; i < len(str); i++ {
		c := str[i]

		if second_to_last_letter == c {
			repeat_with_one_letter_between = true
		}

		prev_two_letter := fmt.Sprintf("%c%c", second_to_last_letter, last_letter)
		if prev_two_letter_map[prev_two_letter] == 0 {
			prev_two_letter_map[prev_two_letter] = i
		}

		two_letter := fmt.Sprintf("%c%c", last_letter, c)
		fmt.Println(two_letter)
		prev_occurrence := prev_two_letter_map[two_letter]
		if prev_occurrence > 0 && prev_occurrence < i {
			repeat_with_no_overlap = true
		}

		second_to_last_letter = last_letter
		last_letter = c
	}

	return repeat_with_one_letter_between && repeat_with_no_overlap
}
