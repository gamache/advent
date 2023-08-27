package main

import (
	// "fmt"
	"os"
	"regexp"
	"strings"
)

func main() {
	Day19()
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

func NamedCaptures(re *regexp.Regexp, haystack string) map[string]string {
	match := re.FindStringSubmatch(haystack)
	if match != nil {
		result := make(map[string]string)
		for i, name := range re.SubexpNames() {
			if i != 0 && name != "" {
				result[name] = match[i]
			}
		}
		return result
	} else {
		return nil
	}
}