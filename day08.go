package main

import "fmt"

func Day08() {
	lines := GetLines("inputs/day08.txt")

	literals := 0
	characters := 0

	for i := range lines {
		line := lines[i]

		for i := 0; i < len(line); i++ {
			literals++

			switch line[i] {
			case '\\':
				// we're in a backslash escape
				switch line[i+1] {
				case 'x':
					// hex literal, e.g. \x2F
					literals += 3
					characters++
					i += 3
				default:
					// single escaped character, e.g. \"
					literals++
					characters++
					i++
				}

			case '"':
				// just a literal, no character

			default:
				characters++
			}
		}
	}

	fmt.Println(literals - characters)

	literals = 0
	characters = 0
	for i := range lines {
		line := lines[i]
		
		// start and end quote
		literals += 2
		
		for i := 0; i < len(line); i++ {
			characters++
			switch line[i] {
			case '"':
				literals += 2
			case '\\':
				literals += 2
			default:
				literals++
			}
		}
	}

	fmt.Println(literals - characters)
}
