package main

import (
	"fmt"
	"regexp"
	"strconv"
)

func Day09() {
	line_re := regexp.MustCompile(
		`(?P<town1>.+) to (?P<town2>.+) = (?P<distance>\d+)`,
	)

	lines := GetLines("inputs/day09.txt")
	distances := make(map[string]map[string]int)

	for i := range lines {
		line := lines[i]
		result := NamedCaptures(line_re, line)
		distance, err := strconv.Atoi(result["distance"])
		Check(err)

		_, present := distances[result["town1"]]
		if !present {
			distances[result["town1"]] = make(map[string]int)
		}
		distances[result["town1"]][result["town2"]] = distance

		_, present = distances[result["town2"]]
		if !present {
			distances[result["town2"]] = make(map[string]int)
		}
		distances[result["town2"]][result["town1"]] = distance
	}

	shortestPath, longestPath := findPaths(distances)
	fmt.Println(shortestPath.distance)
	fmt.Println(longestPath.distance)
}

type TravelState struct {
	town            string
	distance        int // the distance of the path along `visited_towns`
	unvisited_towns []string
	visited_towns   []string // the value of `town` will be the last item
}

// returns shortestPath, longestPath
func findPaths(distances map[string]map[string]int) (TravelState, TravelState) {
	towns := make([]string, 0, len(distances))
	for key := range distances {
		towns = append(towns, key)
	}

	// create initial states, one per start town
	next_states := make([]TravelState, 0, len(towns))

	for _, start_town := range towns {
		visited_towns := []string{start_town}
		unvisited_towns := make([]string, 0, len(towns)-1)

		for _, town := range towns {
			if town != start_town {
				unvisited_towns = append(unvisited_towns, town)
			}
		}

		next_states = append(next_states, TravelState{
			town:            start_town,
			distance:        0,
			unvisited_towns: unvisited_towns,
			visited_towns:   visited_towns,
		})
	}

	for {
		new_next_states := make([]TravelState, 0)

	Next_states:
		for _, state := range next_states {
			if len(state.unvisited_towns) == 0 {
				continue Next_states
			}

		Next_towns:
			for next_town := range distances[state.town] {
				// visit each town exactly once
				for _, town := range state.visited_towns {
					if next_town == town {
						continue Next_towns
					}
				}

				distance := distances[state.town][next_town]

				new_unvisited_towns := make([]string, 0, len(state.unvisited_towns)-1)
				for _, town := range state.unvisited_towns {
					if town != next_town {
						new_unvisited_towns = append(new_unvisited_towns, town)
					}
				}

				new_visited_towns := make([]string, 0, len(state.visited_towns)+1)
				new_visited_towns = append(new_visited_towns, state.visited_towns...)
				new_visited_towns = append(new_visited_towns, next_town)

				new_next_states = append(new_next_states, TravelState{
					town:            next_town,
					distance:        state.distance + distance,
					unvisited_towns: new_unvisited_towns,
					visited_towns:   new_visited_towns,
				})
			}
		}

		if len(new_next_states) == 0 {
			shortest := next_states[0]
			longest := next_states[0]
			for _, state := range next_states {
				if shortest.distance > state.distance {
					shortest = state
				}
				if longest.distance < state.distance {
					longest = state
				}
			}
			return shortest, longest
		} else {
			next_states = new_next_states
		}
	}
}
