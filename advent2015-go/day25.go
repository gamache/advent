package main

import "fmt"

func Day25() {
	code := 20151125
	target_row := 2947
	target_col := 3029

	row := 1
	col := 1
	for {
		code = (code * 252533) % 33554393
		if row == 1 {
			row = col + 1
			col = 1
		} else {
			row--
			col++
		}

		if row == target_row && col == target_col {
			fmt.Println(code)
			break
		}
	}
}
