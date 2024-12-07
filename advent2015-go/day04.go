package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"strings"
)

func Day04() {
	input := GetLines("inputs/day04.txt")[0]

	for i := 1; ; i++ {
		h := md5.New()
		s := fmt.Sprintf("%v%v", input, i)
		io.WriteString(h, s)
		hex := fmt.Sprintf("%x", h.Sum(nil))
		if strings.HasPrefix(hex, "00000") {
			fmt.Println(i)
			break
		}
	}

	// part 2

	for i := 1; ; i++ {
		h := md5.New()
		s := fmt.Sprintf("%v%v", input, i)
		io.WriteString(h, s)
		hex := fmt.Sprintf("%x", h.Sum(nil))
		if strings.HasPrefix(hex, "000000") {
			fmt.Println(i)
			break
		}
	}

}
