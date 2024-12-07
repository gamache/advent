package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	"gonum.org/v1/gonum/mat"
)

func Day24() {
	type Hailstone struct {
		px, py, pz, vx, vy, vz float64
	}

	lines := GetLines("inputs/day24.txt")
	// lines = GetLines("inputs/example24.txt")

	stones := make([]Hailstone, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, " @ ")
		positions := strings.Split(parts[0], ", ")
		velocities := strings.Split(parts[1], ", ")
		ipx, _ := strconv.Atoi(strings.Trim(positions[0], " "))
		ipy, _ := strconv.Atoi(strings.Trim(positions[1], " "))
		ipz, _ := strconv.Atoi(strings.Trim(positions[2], " "))
		ivx, _ := strconv.Atoi(strings.Trim(velocities[0], " "))
		ivy, _ := strconv.Atoi(strings.Trim(velocities[1], " "))
		ivz, _ := strconv.Atoi(strings.Trim(velocities[2], " "))
		px := float64(ipx)
		py := float64(ipy)
		pz := float64(ipz)
		vx := float64(ivx)
		vy := float64(ivy)
		vz := float64(ivz)
		stones[i] = Hailstone{px, py, pz, vx, vy, vz}
	}

	solve2D := func(a, b Hailstone) (float64, float64, bool) {
		matA := mat.NewDense(2, 2, []float64{
			(a.vy / a.vx), -1,
			(b.vy / b.vx), -1,
		})
		vecb := mat.NewVecDense(2, []float64{
			(a.vy*a.px/a.vx - a.py),
			(b.vy*b.px/b.vx - b.py),
		})
		var vecx mat.VecDense
		if err := vecx.SolveVec(matA, vecb); err != nil {
			return float64(0), float64(0), false
		}
		// fmt.Println(mat.Formatted(&vecx))
		return vecx.At(0, 0), vecx.At(1, 0), true
	}

	// part 1

	testMin := float64(200000000000000)
	testMax := float64(400000000000000)
	// testMin = float64(7)
	// testMax = float64(27)

	intersections := 0

	for i, istone := range stones {
		for j := i + 1; j < len(stones); j++ {
			jstone := stones[j]
			x, y, ok := solve2D(istone, jstone)
			inFuture := ((x-istone.px)/istone.vx > 0) && ((x-jstone.px)/jstone.vx > 0)
			// fmt.Printf("%v %v %v %v\n", x, y, ok, inFuture)
			if ok && inFuture &&
				x >= testMin && x <= testMax &&
				y >= testMin && y <= testMax {
				intersections++
			}
		}
	}

	fmt.Println(intersections)

	// part 2
	// there was algebra on paper

	h0 := stones[0]
	coefRowsXY := make([]float64, 16)
	constRowsXY := make([]float64, 4)
	coefRowsXZ := make([]float64, 16)
	constRowsXZ := make([]float64, 4)
	for i, h := range stones[1:5] {
		coefRowsXY[i*4+0] = h.vy - h0.vy
		coefRowsXY[i*4+1] = h0.vx - h.vx
		coefRowsXY[i*4+2] = h0.py - h.py
		coefRowsXY[i*4+3] = h.px - h0.px
		constRowsXY[i] = h.px*h.vy - h.py*h.vx + h0.py*h0.vx - h0.px*h0.vy

		coefRowsXZ[i*4+0] = h.vz - h0.vz
		coefRowsXZ[i*4+1] = h0.vx - h.vx
		coefRowsXZ[i*4+2] = h0.pz - h.pz
		coefRowsXZ[i*4+3] = h.px - h0.px
		constRowsXZ[i] = h.px*h.vz - h.pz*h.vx + h0.pz*h0.vx - h0.px*h0.vz
	}

	matAXY := mat.NewDense(4, 4, coefRowsXY)
	vecbXY := mat.NewVecDense(4, constRowsXY)
	var vecxXY mat.VecDense
	if err := vecxXY.SolveVec(matAXY, vecbXY); err != nil {
		panic("wat")
	}

	matAXZ := mat.NewDense(4, 4, coefRowsXZ)
	vecbXZ := mat.NewVecDense(4, constRowsXZ)
	var vecxXZ mat.VecDense
	if err := vecxXZ.SolveVec(matAXZ, vecbXZ); err != nil {
		panic("wat")
	}

	px := vecxXY.At(0, 0)
	py := vecxXY.At(1, 0)
	pz := vecxXZ.At(1, 0)

	fmt.Println(int(math.Round(px)) + int(math.Round(py)) + int(math.Round(pz)))
}