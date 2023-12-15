package main

import (
	"fmt"
	"strconv"
	"strings"
)

func Day15() {
	input := GetLines("inputs/day15.txt")[0]
	steps := strings.Split(input, ",")

	// part 1

	sum := 0
	for _, step := range steps {
		sum += hash(step)
	}
	fmt.Println(sum)

	// part 2

	type lens struct {
		focLen int
		label  string
	}

	boxes := make([][]lens, 256)
	for _, step := range steps {
		equalsSignIndex := strings.Index(step, "=")
		if equalsSignIndex > -1 {
			label := step[:equalsSignIndex]
			focLen, err := strconv.Atoi(step[equalsSignIndex+1:])
			Check(err)
			boxNum := hash(label)
			box := boxes[boxNum]
			if box == nil {
				box = []lens{}
			}
			foundLabel := false
			for i := range box {
				if box[i].label == label {
					box[i] = lens{focLen, label}
					foundLabel = true
					break
				}
			}
			if !foundLabel {
				box = append(box, lens{focLen, label})
			}
			boxes[boxNum] = box
		} else if strings.HasSuffix(step, "-") {
			label := step[:len(step)-1]
			boxNum := hash(label)
			box := boxes[boxNum]
			if box == nil {
				box = []lens{}
			}
			newBox := []lens{}
			for i := range box {
				if box[i].label != label {
					newBox = append(newBox, box[i])
				}
			}
			boxes[boxNum] = newBox
		} else {
			panic(step)
		}
	}

	sum = 0
	for boxNum, box := range boxes {
		for i, lns := range box {
			power := boxNum + 1
			power *= i + 1
			power *= lns.focLen
			sum += power
		}
	}
	fmt.Println(sum)
}

func hash(s string) int {
	h := 0
	for _, b := range []byte(s) {
		h += int(b)
		h *= 17
		h %= 256
	}
	return h
}