package main

import (
	"fmt"
	"strings"
)

func Day20() {
	const (
		flipFlop    = iota
		inverter    = iota
		broadcaster = iota
	)
	type Node struct {
		name        string
		kind        int
		outputs     []string
		inputLevels map[string]bool
		level       bool
	}
	type Pulse struct {
		source      string
		destination string
		level       bool
	}

	getNodes := func() map[string]Node {
		lines := GetLines("inputs/day20.txt")
		// lines = GetLines("inputs/example20.txt")

		nodes := map[string]Node{}
		inputLevels := map[string]map[string]bool{
			"broadcaster": map[string]bool{
				"button": false,
			},
		}
		for _, line := range lines {
			parts := strings.Split(line, " -> ")
			outputs := strings.Split(parts[1], ", ")
			name := parts[0]
			if strings.HasPrefix(name, "%") {
				name = name[1:]
				nodes[name] = Node{name, flipFlop, outputs, map[string]bool{}, false}
			} else if strings.HasPrefix(name, "&") {
				name = name[1:]
				nodes[name] = Node{name, inverter, outputs, map[string]bool{}, false}
			} else {
				nodes[name] = Node{name, broadcaster, outputs, map[string]bool{}, false}
			}
			for _, output := range outputs {
				inps := inputLevels[output]
				if inps == nil {
					inps = map[string]bool{}
				}
				inps[name] = false
				inputLevels[output] = inps
			}
		}
		for i := range nodes {
			node := nodes[i]
			node.inputLevels = inputLevels[node.name]
			nodes[i] = node
		}
		// fmt.Println(nodes)
		return nodes
	}

	nodes := getNodes()

	pulseCounts := map[bool]int{true: 0, false: 0}

	pressButton := func(nodes map[string]Node, stopAt string) bool {
		pulses := []Pulse{{"button", "broadcaster", false}}
		retVal := false
	PulseLoop:
		for {
			if len(pulses) == 0 {
				break PulseLoop
			}
			pulse := pulses[0]
			if pulse.source == stopAt && pulse.level == true {
				retVal = true
			}

			pulseCounts[pulse.level]++

			pulses = pulses[1:]
			destNode, ok := nodes[pulse.destination]
			if !ok {
				continue PulseLoop
			}

			destNode.inputLevels[pulse.source] = pulse.level
			switch destNode.kind {
			case broadcaster:
				for _, output := range destNode.outputs {
					pulses = append(pulses, Pulse{destNode.name, output, false})
				}
			case flipFlop:
				if pulse.level == false {
					destNode.level = !destNode.level
					for _, output := range destNode.outputs {
						pulses = append(pulses, Pulse{destNode.name, output, destNode.level})
					}
				}
			case inverter:
				allHigh := true
				for _, level := range destNode.inputLevels {
					if level == false {
						allHigh = false
					}
				}
				for _, output := range destNode.outputs {
					pulses = append(pulses, Pulse{destNode.name, output, !allHigh})
				}
			}
			nodes[pulse.destination] = destNode
		}

		return retVal
	}

	// part 1

	for i := 0; i < 1000; i++ {
		pressButton(nodes, "")
	}
	fmt.Println(pulseCounts[true] * pulseCounts[false])

	// part 2
	// absolutely could not have done this without examining the graph by eye

	/*
		graphviz := func(nodes map[string]Node) {
			fmt.Println("digraph {")
			for name, node := range nodes {
				shape := "ellipse"
				if node.kind == inverter {
					shape = "triangle"
				}
				fmt.Printf("%v [shape=%v]\n", name, shape)
				for _, output := range node.outputs {
					fmt.Printf("%v -> %v\n", name, output)
				}
			}
			fmt.Println("}")
		}
		graphviz(nodes)
	*/

	inverters := []string{"qk", "zs", "kr", "kf"}
	factors := make([]int, 4)
	for f, inv := range inverters {
		i := 0
		nodes = getNodes()
		for {
			i++
			if pressButton(nodes, inv) {
				factors[f] = i
				break
			}
		}
	}
	fmt.Println(LCM(factors))

	// 197569289532457 too low, not +1 either
}

// greatest common divisor (GCD) via Euclidean algorithm
func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// find Least Common Multiple (LCM) via GCD
func LCM(ints []int) int {
	if len(ints) == 1 {
		return ints[0]
	}

	a := ints[0]
	b := ints[1]
	result := a * b / GCD(a, b)
	return LCM(append(ints[2:], result))
}
