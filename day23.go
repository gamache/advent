package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day23() {
	lines := GetLines("inputs/day23.txt")

	type inst struct {
		op     string
		reg    string
		offset int
	}

	instRe := regexp.MustCompile(`(?P<op>\S\S\S) (?P<reg>a|b)?(, )?\+?(?P<offset>-?\d+)?`)
	insts := map[int]inst{}
	for i, line := range lines {
		result := NamedCaptures(instRe, line)
		offset := 1
		givenOffset := result["offset"]
		if givenOffset != "" {
			intOffset, err := strconv.Atoi(givenOffset)
			Check(err)
			offset = intOffset
		}

		insts[i] = inst{op: result["op"], reg: result["reg"], offset: offset}
	}

	regs := map[string]int{"a": 0, "b": 0}
	pc := 0
	for {
		inst, present := insts[pc]
		if !present {
			break
		}

		switch inst.op {
		case "hlf":
			regs[inst.reg] /= 2
			pc++
		case "tpl":
			regs[inst.reg] *= 3
			pc++
		case "inc":
			regs[inst.reg]++
			pc++
		case "jmp":
			pc += inst.offset
		case "jie":
			if regs[inst.reg]%2 == 0 {
				pc += inst.offset
			} else {
				pc++
			}
		case "jio":
			if regs[inst.reg] == 1 {
				pc += inst.offset
			} else {
				pc++
			}
		default:
			panic(inst)
		}
	}

	fmt.Println(regs["b"])

	// part 2

	regs = map[string]int{"a": 1, "b": 0}
	pc = 0
	for {
		inst, present := insts[pc]
		if !present {
			break
		}

		switch inst.op {
		case "hlf":
			regs[inst.reg] /= 2
			pc++
		case "tpl":
			regs[inst.reg] *= 3
			pc++
		case "inc":
			regs[inst.reg]++
			pc++
		case "jmp":
			pc += inst.offset
		case "jie":
			if regs[inst.reg]%2 == 0 {
				pc += inst.offset
			} else {
				pc++
			}
		case "jio":
			if regs[inst.reg] == 1 {
				pc += inst.offset
			} else {
				pc++
			}
		default:
			panic(inst)
		}
	}

	fmt.Println(regs["b"])
}
