package main

import (
	"container/heap"
	"fmt"
)

const c coord = {};

type Trail struct {
	coords map[Coord]int
	cur    Coord
	length int
}

type TrailHeap []Trail

func (h TrailHeap) Len() int { return len(h) }
func (h TrailHeap) Less(i, j int) bool {
	return h[i].length > h[j].length /* maxheap/DFS */
	// return h[i].length < h[j].length /* minheap/BFS */
}
func (h TrailHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *TrailHeap) Push(x any)   { *h = append(*h, x.(Trail)) }
func (h *TrailHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func (t Trail) NextTrails(sg StrGrid) []Trail {
	nextTrails := []Trail{}

	// up
	c := Coord{t.cur.Row - 1, t.cur.Col}
	sgc := sg.Grid[c]
	_, visited := t.coords[c]
	if !visited {
		newCoords := map[Coord]int{c: t.length + 1}
		for c, i := range t.coords {
			newCoords[c] = i
		}
		if sgc == "." {
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		} else if sgc == "^" {
			c2 := Coord{c.Row - 1, c.Col}
			_, visited2 := newCoords[c2]
			if !visited2 {
				newCoords[c2] = t.length + 2
				t := Trail{newCoords, c2, t.length + 2}
				nextTrails = append(nextTrails, t)
			}
		}
	}

	// down
	c = Coord{t.cur.Row + 1, t.cur.Col}
	sgc = sg.Grid[c]
	_, visited = t.coords[c]
	if !visited {
		newCoords := map[Coord]int{c: t.length + 1}
		for c, i := range t.coords {
			newCoords[c] = i
		}
		if sgc == "." {
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		} else if sgc == "v" {
			c2 := Coord{c.Row + 1, c.Col}
			_, visited2 := newCoords[c2]
			if !visited2 {
				newCoords[c2] = t.length + 2
				t := Trail{newCoords, c2, t.length + 2}
				nextTrails = append(nextTrails, t)
			}
		}
	}

	// left
	c = Coord{t.cur.Row, t.cur.Col - 1}
	sgc = sg.Grid[c]
	_, visited = t.coords[c]
	if !visited {
		newCoords := map[Coord]int{c: t.length + 1}
		for c, i := range t.coords {
			newCoords[c] = i
		}
		if sgc == "." {
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		} else if sgc == "<" {
			c2 := Coord{c.Row, c.Col - 1}
			_, visited2 := newCoords[c2]
			if !visited2 {
				newCoords[c2] = t.length + 2
				t := Trail{newCoords, c2, t.length + 2}
				nextTrails = append(nextTrails, t)
			}
		}
	}

	// right
	c = Coord{t.cur.Row, t.cur.Col + 1}
	sgc = sg.Grid[c]
	_, visited = t.coords[c]
	if !visited {
		newCoords := map[Coord]int{c: t.length + 1}
		for c, i := range t.coords {
			newCoords[c] = i
		}
		if sgc == "." {
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		} else if sgc == ">" {
			c2 := Coord{c.Row, c.Col + 1}
			_, visited2 := newCoords[c2]
			if !visited2 {
				newCoords[c2] = t.length + 2
				t := Trail{newCoords, c2, t.length + 2}
				nextTrails = append(nextTrails, t)
			}
		}
	}

	if len(nextTrails) == 1 {
		return nextTrails[0].NextTrails2(sg)
	}
	return nextTrails
}

func (t Trail) NextTrails2(sg StrGrid) []Trail {
	nextTrails := []Trail{}

	// up
	c := Coord{t.cur.Row - 1, t.cur.Col}
	sgc, inBounds := sg.Grid[c]
	_, visited := t.coords[c]
	if inBounds && !visited {
		if sgc != "#" {
			newCoords := map[Coord]int{c: t.length + 1}
			for c, i := range t.coords {
				newCoords[c] = i
			}
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		}
	}

	// down
	c = Coord{t.cur.Row + 1, t.cur.Col}
	sgc, inBounds = sg.Grid[c]
	_, visited = t.coords[c]
	if inBounds && !visited {
		if sgc != "#" {
			newCoords := map[Coord]int{c: t.length + 1}
			for c, i := range t.coords {
				newCoords[c] = i
			}
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		}
	}

	// left
	c = Coord{t.cur.Row, t.cur.Col - 1}
	sgc, inBounds = sg.Grid[c]
	_, visited = t.coords[c]
	if inBounds && !visited {
		if sgc != "#" {
			newCoords := map[Coord]int{c: t.length + 1}
			for c, i := range t.coords {
				newCoords[c] = i
			}
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		}
	}

	// right
	c = Coord{t.cur.Row, t.cur.Col + 1}
	sgc, inBounds = sg.Grid[c]
	_, visited = t.coords[c]
	if inBounds && !visited {
		if sgc != "#" {
			newCoords := map[Coord]int{c: t.length + 1}
			for c, i := range t.coords {
				newCoords[c] = i
			}
			t := Trail{newCoords, c, t.length + 1}
			nextTrails = append(nextTrails, t)
		}
	}

	return nextTrails
}

func Day23() {
	sg := FromLines(GetLines("inputs/day23.txt"))
	// sg = FromLines(GetLines("inputs/example23.txt"))

	start := Coord{0, 1}
	end := Coord{sg.RowMax, sg.ColMax - 1}
	startTrail := Trail{coords: map[Coord]int{start: 0}, cur: start, length: 0}

	part1 := func() {
		bestTrailToEnd := startTrail
		trails := &TrailHeap{startTrail}
		heap.Init(trails)

		for {
			if len(*trails) == 0 {
				break
			}
			trail := heap.Pop(trails).(Trail)

			if trail.cur == end {
				if trail.length > bestTrailToEnd.length {
					bestTrailToEnd = trail
				}
			} else {
				nextTrails := trail.NextTrails(sg)
				for _, t := range nextTrails {
					heap.Push(trails, t)
				}
			}
		}
		fmt.Println(bestTrailToEnd.length)
	}
	if part1 == nil {
		panic("shut up about unused variables, you miserable fucking compiler")
	}

	part2 := func() {
		bestTrails := map[Coord]Trail{start: startTrail}
		trails := &TrailHeap{startTrail}
		heap.Init(trails)

		for {
			if len(*trails) == 0 {
				break
			} else {
				// fmt.Println(len(*trails))
			}

			trail := heap.Pop(trails).(Trail)
			fmt.Printf("%v\t%v\t%v\n", trail.length, len(*trails), bestTrails[end].length)

			if trail.cur != end {
				nextTrails := trail.NextTrails2(sg)
				for _, t := range nextTrails {
					heap.Push(trails, t)
					if t.cur == end && t.length > bestTrails[t.cur].length {
						bestTrails[t.cur] = t
					}
				}
			}
		}
		fmt.Println(bestTrails[end].length)
	}

	// part1()

	// 5102 too low
	// 6034 too low
	part2()
}
