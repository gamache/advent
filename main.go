package main

import (
	// "fmt"
	"os"
	"strings"
)

func main() {
	Day06()
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func GetLines(filename string) []string {
	data, err := os.ReadFile(filename)
	Check(err)

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	return lines
}

type Coord struct {
	x int
	y int
}