package main

import (
	// "fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func GetLines(filename string) []string {
	data, err := os.ReadFile(filename)
	check(err)

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	return lines
}

func main() {
	lines := GetLines("inputs/day01.txt")
	Day01(lines)
}
