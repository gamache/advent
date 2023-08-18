package main

import (
	// "fmt"
	"os"
	"strings"
)

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

func main() {
	Day04()
}
