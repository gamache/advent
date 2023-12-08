package main

import (
	"fmt"
	"regexp"
	"strings"
)

func Day08() {
	lines := GetLines("inputs/day08.txt")
	nodeRe := regexp.MustCompile(`[A-Z]{3}`)

	directions := strings.Split(lines[0], "")

	// e.g., nextNode := nodes["AAA"]["L"]
	nodes := make(map[string]map[string]string, len(lines[2:]))
	for _, line := range lines[2:] {
		threeNodes := nodeRe.FindAllString(line, -1)
		nodes[threeNodes[0]] = map[string]string{
			"L": threeNodes[1],
			"R": threeNodes[2],
		}
	}

	// part 1

	steps := 0
	startNode := "AAA"
Steps:
	for {
		for _, dir := range directions {
			steps++
			startNode = nodes[startNode][dir]
			if startNode == "ZZZ" {
				break Steps
			}
		}
	}

	fmt.Println(steps)

	// part 2

	startNodes := []string{}
	for node := range nodes {
		if strings.HasSuffix(node, "A") {
			startNodes = append(startNodes, node)
		}
	}

	stepsUntilFirstZ := make(map[string]int, len(nodes))
	firstZ := make(map[string]string, len(nodes))
	for node := range nodes {
		steps := 0
		origNode := node
	DirsLoop:
		for {
			for _, dir := range directions {
				steps++
				node = nodes[node][dir]
				if strings.HasSuffix(node, "Z") {
					stepsUntilFirstZ[origNode] = steps
					firstZ[origNode] = node
					break DirsLoop
				}
			}
		}
	}

	/*
		for _, startNode := range startNodes {
			fmt.Printf(
				"%v to %v in %v (%.3f)\n",
				startNode,
				firstZ[startNode],
				stepsUntilFirstZ[startNode],
				float32(stepsUntilFirstZ[startNode])/float32(len(directions)),
			)
		}

		// holy shit they're all multiples of len(directions) and they're
		// all prime numbers oh fuck oh fuck
	*/

	trips := 1
	for _, startNode := range startNodes {
		trips *= stepsUntilFirstZ[startNode] / len(directions)
	}

	fmt.Println(trips * len(directions))
}
