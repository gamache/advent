package main

import (
	"encoding/json"
	"fmt"
)

func Day12() {
	input := GetLines("inputs/day12.txt")[0]
	
	var jsonValue any
	json.Unmarshal([]byte(input), &jsonValue)
	
	fmt.Println(getSum(jsonValue))
	fmt.Println(getSumNoRed(jsonValue))
}

func getSum(jv any) int {
	switch jv.(type) {
	case float64:
		return int(jv.(float64))
	case []any:
		sum := 0
		for _, v := range jv.([]any) {
			sum += getSum(v)
		}
		return sum
	case map[string]any:
		sum := 0
		for _, v := range jv.(map[string]any) {
			sum += getSum(v)
		}
		return sum
	default:
		return 0
	}
}

func getSumNoRed(jv any) int {
	switch jv.(type) {
	case float64:
		return int(jv.(float64))
	case []any:
		sum := 0
		for _, v := range jv.([]any) {
			sum += getSumNoRed(v)
		}
		return sum
	case map[string]any:
		sum := 0
		for _, v := range jv.(map[string]any) {
			switch v.(type) {
		    case string:
				if v.(string) == "red" { return 0 }
			}
			sum += getSumNoRed(v)
		}
		return sum
	default:
		return 0
	}
}