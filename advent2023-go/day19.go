package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Constraint struct {
	rating   string
	min, max int
}

// keyed by rating
type Constraints map[string]Constraint

func (cs Constraints) Merge(c Constraint) {
	if c.rating == "" {
		return
	}
	oldC, ok := cs[c.rating]
	if !ok {
		cs[c.rating] = c
	} else {
		if c.min != 0 && (oldC.min == 0 || oldC.min < c.min) {
			oldC.min = c.min
		}
		if c.max != 0 && (oldC.max == 0 || oldC.max > c.max) {
			oldC.max = c.max
		}
		cs[c.rating] = oldC
	}
}
func (cs Constraints) Count() int {
	count := 1
	for _, rating := range []string{"x", "m", "a", "s"} {
		c := cs[rating]
		min := c.min
		if min == 0 {
			min = 1
		}
		max := c.max
		if max == 0 {
			max = 4000
		}
		if max >= min {
			count *= max - min + 1
		}
	}
	return count
}
func (cs Constraints) Clone() Constraints {
	newCs := make(Constraints)
	for k, v := range cs {
		newCs[k] = v
	}
	return newCs
}

func Day19() {
	data, err := os.ReadFile("inputs/day19.txt")
	Check(err)

	type Part map[string]int
	type Rule struct {
		force    bool
		rating   string
		op       string
		value    int
		workflow string
	}
	workflowRe := regexp.MustCompile(`([a-z]+)\{(.+)\}`)
	ruleRe := regexp.MustCompile(`([xmas])([<>])(\d+):([a-zAR]+)`)

	workflows := map[string][]Rule{}

	inputChunks := strings.Split(string(data), "\n\n")
	for _, wfstr := range strings.Split(inputChunks[0], "\n") {
		rules := []Rule{}
		wfparts := workflowRe.FindStringSubmatch(wfstr)
		wfname := wfparts[1]
		for _, rule := range strings.Split(wfparts[2], ",") {
			ruleParts := ruleRe.FindStringSubmatch(rule)
			if ruleParts == nil {
				rules = append(rules, Rule{true, "", "", 0, rule})
			} else {
				rating := ruleParts[1]
				op := ruleParts[2]
				value, _ := strconv.Atoi(ruleParts[3])
				workflow := ruleParts[4]
				rules = append(rules, Rule{false, rating, op, value, workflow})
			}
		}
		workflows[wfname] = rules
	}

	// fmt.Println(workflows)

	numRe := regexp.MustCompile(`\d+`)
	parts := []Part{}
	for _, partStr := range strings.Split(strings.Trim(inputChunks[1], "\n "), "\n") {
		ratings := numRe.FindAllString(partStr, -1)
		if ratings == nil {
			panic(partStr)
		}
		x, _ := strconv.Atoi(ratings[0])
		m, _ := strconv.Atoi(ratings[1])
		a, _ := strconv.Atoi(ratings[2])
		s, _ := strconv.Atoi(ratings[3])
		parts = append(parts, Part{"x": x, "m": m, "a": a, "s": s})
	}

	// fmt.Println(parts)

	var work func(p Part, wfname string) string
	work = func(p Part, wfname string) string {
		for _, rule := range workflows[wfname] {
			match := rule.force
			if !match && rule.op == ">" && p[rule.rating] > rule.value {
				match = true
			}
			if !match && rule.op == "<" && p[rule.rating] < rule.value {
				match = true
			}
			if match {
				if rule.workflow == "A" || rule.workflow == "R" {
					return rule.workflow
				}
				return work(p, rule.workflow)
			}
		}
		panic("that did not work")
	}

	sum := 0
	for _, part := range parts {
		if work(part, "in") == "A" {
			for _, v := range part {
				sum += v
			}
		}
	}
	fmt.Println(sum)

	/*
		graphviz := func() {
			fmt.Println("digraph {")
			for name, wf := range workflows {
				for _, rule := range wf {
					fmt.Printf(
						"%v -> %v [comment=\"%v%v%v\"]\n",
						name,
						rule.workflow,
						rule.rating,
						rule.op,
						rule.value,
					)
				}
			}
			fmt.Println("}")
		}
		graphviz()
	*/

	// part 2

	var countWays func(wfname string, cs Constraints) int
	countWays = func(wfname string, cs Constraints) int {
		fmt.Println(cs)
		if wfname == "R" {
			return 0
		}
		if wfname == "A" {
			return cs.Count()
		}

		count := 0
		for _, rule := range workflows[wfname] {
			newCs := cs.Clone()
			c := Constraint{rating: rule.rating}
			csc := Constraint{rating: rule.rating}

			if rule.op == ">" {
				c.min = rule.value + 1
				csc.max = rule.value
			} else if rule.op == "<" {
				c.max = rule.value - 1
				csc.min = rule.value
			}
			newCs.Merge(c)
			cs.Merge(csc)
			count += countWays(rule.workflow, newCs)
		}
		return count
	}

	fmt.Println(countWays("in", Constraints{}))

}
