package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

const CARDS1 string = "23456789TJQKA"

// returns an int representing this hand's relative strength; higher is better
func rankHand1(hand string) int {
	cards := strings.Split(hand, "")

	cardValues := make([]int, len(cards))
	cardCount := make([]int, len(CARDS1))
	for i, card := range cards {
		cardIndex := strings.Index(CARDS1, card)
		cardValues[i] = cardIndex
		cardCount[cardIndex]++
	}

	fives := 0
	fours := 0
	threes := 0
	pairs := 0

	for _, count := range cardCount {
		switch count {
		case 5:
			fives++
		case 4:
			fours++
		case 3:
			threes++
		case 2:
			pairs++
		}
	}

	value := 0

	if fives > 0 {
		value += 1 << 30
	} else if fours > 0 {
		value += 1 << 29
	} else if threes > 0 && pairs > 0 {
		value += 1 << 28
	} else if threes > 0 {
		value += 1 << 27
	} else if pairs == 2 {
		value += 1 << 26
	} else if pairs == 1 {
		value += 1 << 25
	}

	value += (1 << 16) * cardValues[0]
	value += (1 << 12) * cardValues[1]
	value += (1 << 8) * cardValues[2]
	value += (1 << 4) * cardValues[3]
	value += (1 << 0) * cardValues[4]

	return value
}

type rankedHand struct {
	hand string
	bid  int
	rank int
}

func Day07() {
	lines := GetLines("inputs/day07.txt")
	day07part1(lines)
	day07part2(lines)
}

func day07part1(lines []string) {
	hands := make([]rankedHand, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, " ")
		hand := parts[0]
		bid, err := strconv.Atoi(parts[1])
		Check(err)
		rank := rankHand1(hand)
		hands[i] = rankedHand{hand, bid, rank}
	}

	sort.Slice(hands, func(i, j int) bool {
		return hands[i].rank < hands[j].rank
	})

	winnings := 0
	for i, rankedHand := range hands {
		winnings += rankedHand.bid * (i + 1)
	}
	fmt.Println(winnings)
}

const CARDS2 string = "J23456789TQKA"

// returns an int representing this hand's relative strength; higher is better
func rankHand2(hand string) int {
	cards := strings.Split(hand, "")

	cardValues := make([]int, len(cards))
	cardCount := make([]int, len(CARDS2))
	for i, card := range cards {
		cardIndex := strings.Index(CARDS2, card)
		cardValues[i] = cardIndex
		cardCount[cardIndex]++
	}

	fives := 0
	fours := 0
	threes := 0
	pairs := 0
	jokers := cardCount[0]

	for _, count := range cardCount[1:] {
		switch count {
		case 5:
			fives++
		case 4:
			fours++
		case 3:
			threes++
		case 2:
			pairs++
		}
	}

	value := 0

	if fives == 1 ||
		(fours == 1 && jokers == 1) ||
		(threes == 1 && jokers == 2) ||
		(pairs == 1 && jokers == 3) ||
		jokers >= 4 {
		value += 1 << 30
	} else if fours == 1 ||
		(threes == 1 && jokers == 1) ||
		(pairs == 1 && jokers == 2) ||
		jokers == 3 {
		value += 1 << 29
	} else if (threes == 1 && pairs == 1) ||
		(pairs == 2 && jokers == 1) ||
		(pairs == 1 && jokers == 2) {
		value += 1 << 28
	} else if threes == 1 ||
		(pairs == 1 && jokers == 1) ||
		jokers == 2 {
		value += 1 << 27
	} else if pairs == 2 || (pairs == 1 && jokers == 1) {
		value += 1 << 26
	} else if pairs == 1 || jokers == 1 {
		value += 1 << 25
	}

	value += (1 << 16) * cardValues[0]
	value += (1 << 12) * cardValues[1]
	value += (1 << 8) * cardValues[2]
	value += (1 << 4) * cardValues[3]
	value += (1 << 0) * cardValues[4]

	return value
}

func day07part2(lines []string) {
	hands := make([]rankedHand, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, " ")
		hand := parts[0]
		bid, err := strconv.Atoi(parts[1])
		Check(err)
		rank := rankHand2(hand)
		hands[i] = rankedHand{hand, bid, rank}
	}

	sort.Slice(hands, func(i, j int) bool {
		return hands[i].rank < hands[j].rank
	})

	winnings := 0
	for i, rankedHand := range hands {
		winnings += rankedHand.bid * (i + 1)
	}
	fmt.Println(winnings)
}
