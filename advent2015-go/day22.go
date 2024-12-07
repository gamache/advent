package main

import (
	"container/heap"
	"fmt"
	"regexp"
	"strconv"
)

type spell struct {
	name     string
	cost     int
	duration int
	setup    func(bs *battlestate)
	tick     func(bs *battlestate)
	teardown func(bs *battlestate)
}

var	allSpells = []spell{
	spell{
		name:     "Magic missile",
		cost:     53,
		duration: 0,
		setup:    func(bs *battlestate) { bs.theirHp -= 4 },
		tick:     func(bs *battlestate) {},
		teardown: func(bs *battlestate) {},
	},
	spell{
		name:     "Drain",
		cost:     73,
		duration: 0,
		setup:    func(bs *battlestate) { bs.theirHp -= 2; bs.myHp += 2 },
		tick:     func(bs *battlestate) {},
		teardown: func(bs *battlestate) {},
	},
	spell{
		name:     "Shield",
		cost:     113,
		duration: 6,
		setup:    func(bs *battlestate) { bs.myArmor += 7 },
		tick:     func(bs *battlestate) {},
		teardown: func(bs *battlestate) { bs.myArmor -= 7 },
	},
	spell{
		name:     "Poison",
		cost:     173,
		duration: 6,
		setup:    func(bs *battlestate) {},
		tick:     func(bs *battlestate) { bs.theirHp -= 3 },
		teardown: func(bs *battlestate) {},
	},
	spell{
		name:     "Recharge",
		cost:     229,
		duration: 5,
		setup:    func(bs *battlestate) {},
		tick:     func(bs *battlestate) { bs.myMana += 101 },
		teardown: func(bs *battlestate) {},
	},
}

type battlestate struct {
	spells    []spell
	manaSpent int
	theirHp   int
	myHp      int
	myArmor   int
	myMana    int
}

type bsHeap []battlestate

func (h bsHeap) Len() int           { return len(h) }
func (h bsHeap) Less(i, j int) bool { return h[i].manaSpent < h[j].manaSpent }
func (h bsHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *bsHeap) Push(x any)        { *h = append(*h, x.(battlestate)) }
func (h *bsHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func Day22() {
	lines := GetLines("inputs/day22.txt")
	digitRe := regexp.MustCompile(`(?P<n>\d+)`)
	result0 := NamedCaptures(digitRe, lines[0])
	enemyHp, err := strconv.Atoi(result0["n"])
	Check(err)
	result1 := NamedCaptures(digitRe, lines[1])
	enemyDamage, err := strconv.Atoi(result1["n"])
	Check(err)

	battlestates := &bsHeap{}
	heap.Init(battlestates)
	heap.Push(battlestates, battlestate{
		spells:    []spell{},
		manaSpent: 0,
		theirHp:   enemyHp,
		myHp:      50,
		myArmor:   0,
		myMana:    500,
	})

Battlestates:
	for {
		if len(*battlestates) == 0 {
			panic("empty!")
		}

		bs := heap.Pop(battlestates).(battlestate)

	Spells:
		for _, sp := range allSpells {
			if sp.cost > bs.myMana {
				continue Spells
			}
			
			// can't run the same spell more than once at a time
			for _, s := range bs.spells {
				if sp.name == s.name && s.duration > 1 {
					continue Spells
				}
			}

			newBs := bs
			newBs.manaSpent += sp.cost
			newBs.myMana -= sp.cost
			
			sp.setup(&newBs)
			
			newSpells := []spell{sp}
			for _, s := range newBs.spells {
				if s.duration > 0 {
					s.tick(&newBs)
					newSpell := s
					newSpell.duration--
					newSpells = append(newSpells, newSpell)
				} else if s.duration == 0 {
					s.teardown(&newBs)
				}
			}
			
			if newBs.theirHp <= 0 {
				fmt.Println(newBs.manaSpent)
				break Battlestates
			}
			
			newSpells2 := []spell{}
			for _, s := range newSpells {
				if s.duration > 0 {
					s.tick(&newBs)
					newSpell := s
					newSpell.duration--
					newSpells2 = append(newSpells2, newSpell)
				} else if s.duration == 0 {
					s.teardown(&newBs)
				}
			}
			
			myDamage := enemyDamage - newBs.myArmor
			if myDamage < 0 {
				myDamage = 0
			}
			newBs.myHp -= myDamage

			if newBs.myHp <= 0 {
				continue Spells
			}
			if newBs.theirHp <= 0 {
				fmt.Println(newBs.manaSpent)
				break Battlestates
			}

			newBs.spells = newSpells2
			heap.Push(battlestates, newBs)
		}
	}

	// part 2

	battlestates = &bsHeap{}
	heap.Init(battlestates)
	heap.Push(battlestates, battlestate{
		spells:    []spell{},
		manaSpent: 0,
		theirHp:   enemyHp,
		myHp:      50,
		myArmor:   0,
		myMana:    500,
	})

Battlestates2:
	for {
		if len(*battlestates) == 0 {
			panic("empty!")
		}

		bs := heap.Pop(battlestates).(battlestate)

	Spells2:
		for _, sp := range allSpells {
			if sp.cost > bs.myMana {
				continue Spells2
			}
			
			// can't run the same spell more than once at a time
			for _, s := range bs.spells {
				if sp.name == s.name && s.duration > 1 {
					continue Spells2
				}
			}

			newBs := bs
			newBs.manaSpent += sp.cost
			newBs.myMana -= sp.cost
			
			newBs.myHp--
			if newBs.myHp <= 0 {
				continue Spells2
			}
			
			sp.setup(&newBs)
			
			newSpells := []spell{sp}
			for _, s := range newBs.spells {
				if s.duration > 0 {
					s.tick(&newBs)
					newSpell := s
					newSpell.duration--
					newSpells = append(newSpells, newSpell)
				} else if s.duration == 0 {
					s.teardown(&newBs)
				}
			}
			
			if newBs.theirHp <= 0 {
				fmt.Println(newBs.manaSpent)
				break Battlestates2
			}
			
			newSpells2 := []spell{}
			for _, s := range newSpells {
				if s.duration > 0 {
					s.tick(&newBs)
					newSpell := s
					newSpell.duration--
					newSpells2 = append(newSpells2, newSpell)
				} else if s.duration == 0 {
					s.teardown(&newBs)
				}
			}
			
			myDamage := enemyDamage - newBs.myArmor
			if myDamage < 0 {
				myDamage = 0
			}
			newBs.myHp -= myDamage

			if newBs.myHp <= 0 {
				continue Spells2
			}
			if newBs.theirHp <= 0 {
				fmt.Println(newBs.manaSpent)
				break Battlestates2
			}

			newBs.spells = newSpells2
			heap.Push(battlestates, newBs)
		}
	}
}