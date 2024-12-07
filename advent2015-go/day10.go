package main

import (
	"bytes"
	"fmt"
	"regexp"
)

func Day10() {
	lines := GetLines("inputs/day10.txt")

	str := lines[0]

	for i := 0; i < 50; i++ {
		if i == 40 {
			fmt.Println(len(str))
		}
		str = lookSee(str)
	}
	fmt.Println(len(str))
}

var seqRe = regexp.MustCompile(`(0+|1+|2+|3+|4+|5+|6+|7+|8+|9+)`)

func lookSee(str string) string {
	var output bytes.Buffer
	matches := seqRe.FindAllStringSubmatch(str, -1)
	for _, match := range matches {
		seq := match[0]
		output.WriteString(fmt.Sprintf("%v%v", len(seq), string(seq[0])))
	}
	return output.String()
}
