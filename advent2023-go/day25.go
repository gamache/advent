package main

import (
	"fmt"
	"strings"
)

func Day25() {
	lines := GetLines("inputs/day25.txt")
	// lines = GetLines("inputs/example25.txt")

	type Edge struct {
		a, b string
	}

	graph := map[string]map[string]bool{}
	edgesMap := map[Edge]bool{}
	for _, line := range lines {
		parts := strings.Split(line, ": ")
		a := parts[0]
		for _, b := range strings.Split(parts[1], " ") {
			if graph[a] == nil {
				graph[a] = map[string]bool{}
			}
			if graph[b] == nil {
				graph[b] = map[string]bool{}
			}
			graph[a][b] = true
			graph[b][a] = true

			if a < b {
				edgesMap[Edge{a, b}] = true
			} else {
				edgesMap[Edge{b, a}] = true
			}
		}
	}
	edges := make([]Edge, len(edgesMap))
	e := 0
	for edge := range edgesMap {
		edges[e] = edge
		e++
	}

	/*
		// once again, visualization does the hard part
		graphviz := func() {
			fmt.Println("graph {")
			for _, edge := range edges {
				fmt.Printf("%v -- %v\n", edge.a, edge.b)
			}

			fmt.Println("}")
		}
		graphviz()
	*/

	countPartitionSizes := func(removed []Edge) []int {
		startNodes := map[string]bool{}
		for _, r := range removed {
			startNodes[r.a] = true
			startNodes[r.b] = true
		}
		partitionMap := map[string]int{}
		partN := -1

	StartNodes:
		for node := range startNodes {
			_, ok := partitionMap[node]
			if ok {
				continue StartNodes
			}
			partN++
			searchNodes := []string{node}

		Search:
			for {
				if len(searchNodes) == 0 {
					break Search
				}

				next := searchNodes[0]
				searchNodes = searchNodes[1:]

				_, ok := partitionMap[next]
				if ok {
					continue Search
				}
				partitionMap[next] = partN

			Neighbors:
				for neighbor := range graph[next] {
					for _, edge := range removed {
						if (next == edge.a && neighbor == edge.b) ||
							(next == edge.b && neighbor == edge.a) {
							continue Neighbors
						}
					}

					_, ok := partitionMap[neighbor]
					if !ok {
						searchNodes = append(searchNodes, neighbor)
					}
				}
			}
		}
		maxPart := 0
		countMap := map[int]int{}
		for _, part := range partitionMap {
			countMap[part]++
			if maxPart < part {
				maxPart = part

			}
		}

		counts := make([]int, len(countMap))
		for p := 0; p <= maxPart; p++ {
			counts[p] = countMap[p]
		}
		return counts
	}

	// thanks to visualizing the graph with neato/dot/graphviz/whatever,
	// the edges to cut are obvious
	cuts := []Edge{
		{"krx", "lmg"},
		{"tnr", "vzb"},
		{"tqn", "tvf"},
	}
	counts := countPartitionSizes(cuts)
	if len(counts) == 2 {
		fmt.Println(counts[0] * counts[1])
	} else {
		fmt.Println("SHIT")
	}
}
