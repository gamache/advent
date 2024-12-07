package main

import (
	"fmt"
	"strconv"
)

func Day20() {
	input, err := strconv.Atoi(GetLines("inputs/day20.txt")[0])
	Check(err)

	c := make(chan bool)

	part1 := func() {
		n := 1
		for {
			sum := 10 * n // elf n's delivery
			for i := 1; i <= n/2; i++ {
				if n%i == 0 {
					sum += 10 * i
				}
			}

			if sum > input {
				break
			}

			n++
		}

		fmt.Printf("part 1: %v\n", n)
		c <- true
	}

	part2 := func() {
		n := 1
		for {
			sum := 11 * n // elf n's delivery
			for i := 1; i <= n/2; i++ {
				if n%i == 0 && n/i <= 50 {
					sum += 11 * i
				}
			}

			if sum > input {
				break
			}

			n++
		}

		fmt.Printf("part 2: %v\n", n)
		c <- true
	}

	go part1()
	go part2()
	<-c
	<-c
}
