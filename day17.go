package main

import (
	"container/heap"
	"fmt"
	"strconv"
)

type Path struct {
	nodes   []Coord
	cost    int
	estCost int
}

func (p Path) PathStr() string {
	return fmt.Sprint(p.nodes[len(p.nodes)-4:])
}

type PathHeap []Path

func (h PathHeap) Len() int { return len(h) }
func (h PathHeap) Less(i, j int) bool {
	return h[i].estCost < h[j].estCost
}
func (h PathHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *PathHeap) Push(x any)   { *h = append(*h, x.(Path)) }
func (h *PathHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func AbsInt(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func Day17() {
	sg := FromLines(GetLines("inputs/day17.txt"))
	sg = FromLines(GetLines("inputs/example17.txt"))

	// let's do A*

	nextCoords := func(p Path) []Coord {
		next := []Coord{}

		cur := p.nodes[len(p.nodes)-1]
		prev := p.nodes[len(p.nodes)-2]
		prev3 := p.nodes[len(p.nodes)-4]

		above := Coord{cur.Row - 1, cur.Col}
		below := Coord{cur.Row + 1, cur.Col}
		left := Coord{cur.Row, cur.Col - 1}
		right := Coord{cur.Row, cur.Col + 1}

		if above != prev && prev3.Row-cur.Row != 3 {
			next = append(next, above)
		}
		if below != prev && prev3.Row-cur.Row != -3 {
			next = append(next, below)
		}
		if left != prev && prev3.Col-cur.Col != 3 {
			next = append(next, left)
		}
		if right != prev && prev3.Col-cur.Col != -3 {
			next = append(next, right)
		}
		// fmt.Println(next)
		return next
	}

	visualize := func(path Path) {
		vg := StrGrid{
			sg.RowMax,
			sg.ColMax,
			map[Coord]string{},
		}
		for i, crd := range path.nodes {
			n := (i - 3) % 10
			vg.Grid[crd] = fmt.Sprint(n)
		}
		vg.Print()
	}

	aStar := func(costs StrGrid, start, end Coord) Path {
		// heuristic for path cost must never overestimate
		h := func(crd Coord) int {
			r := AbsInt(end.Row - crd.Row)
			c := AbsInt(end.Col - crd.Col)
			// return 0 Dijkstra mode
			return r + c
		}

		startPath := Path{
			nodes: []Coord{
				// it's easier in nextCoords if there are always three previous nodes
				{-5, -5},
				{-5, -5},
				{-5, -5},
				start,
			},
			cost:    0,
			estCost: h(start),
		}
		paths := &PathHeap{startPath}
		pathSet := map[string]int{startPath.PathStr(): 0}
		fmt.Println(startPath.PathStr())

		heap.Init(paths)
		cheapestPathTo := map[Coord]Path{}
		cheapestPathTo[start] = startPath

		// Paths:
		for {
			if paths.Len() == 0 {
				fmt.Println("oops")
				break
			}
			path := heap.Pop(paths).(Path)
			delete(pathSet, fmt.Sprint(path.nodes))

			fmt.Printf("%v\t%v\t%v\n", paths.Len(), path.cost, path.estCost)
			// visualize(path)

			if path.nodes[len(path.nodes)-1] == end {
				fmt.Println("we're there")
				break
			}

			// Neighbors:
			for _, crd := range nextCoords(path) {
				// fmt.Printf("Neighbor %v\n", crd)
				costStr, ok := costs.Grid[crd]
				if !ok {
					// fmt.Printf("out of bounds %v\n", crd)
					continue // out of bounds
				}
				cost, err := strconv.Atoi(costStr)
				Check(err)
				newNodes := make([]Coord, len(path.nodes))
				copy(newNodes, path.nodes)

				newPath := Path{
					nodes:   append(newNodes, crd),
					cost:    path.cost + cost,
					estCost: path.cost + cost + h(crd),
				}

				pushIt := crd == end
				cheapest, ok := cheapestPathTo[crd]
				if !ok || newPath.cost < cheapest.cost {
					pushIt = true
					cheapestPathTo[crd] = newPath
				}
				// look ahead one move, to counteract must-turn situations
				for _, next := range nextCoords(newPath) {
					// fmt.Printf("Next %v\n", next)
					costStr, ok := costs.Grid[next]
					if !ok {
						continue // out of bounds
					}
					cost, err := strconv.Atoi(costStr)
					Check(err)
					nextNodes := make([]Coord, len(newPath.nodes))
					copy(nextNodes, newPath.nodes)
					nextPath := Path{
						nodes:   append(nextNodes, next),
						cost:    newPath.cost + cost,
						estCost: newPath.cost + cost + h(next),
					}
					cheapest, ok := cheapestPathTo[next]
					// fmt.Printf("our cost %v cheapest %v\n", nextPath.cost, cheapest.cost)
					if !ok || nextPath.cost < cheapest.cost || next == end {
						//cheapestPathTo[next] = nextPath
						pushIt = true
						// fmt.Printf("push it %v %v\n", next, nextPath.estCost)
					}
				}
				if pushIt || crd == end {
					oldCost, ok := pathSet[newPath.PathStr()]
					if !ok || oldCost > newPath.cost {
						// fmt.Printf("pushing %v\n", newPath)
						pathSet[newPath.PathStr()] = newPath.cost
						heap.Push(paths, newPath)
					}
				}
			}
		}

		return cheapestPathTo[end]
	}

	path := aStar(sg, Coord{0, 0}, Coord{sg.RowMax, sg.ColMax})
	visualize(path)

	// 2115 too high
	// 837 too high
	// 825 too high
	// no it is not 827 that comes up too
	// it is not 819, which is what I find with Dijkstra
	fmt.Println(path.cost)

}
