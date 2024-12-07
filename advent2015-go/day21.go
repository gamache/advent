package main

import (
	"fmt"
	"regexp"
	"sort"
	"strconv"
)

func Day21() {
	lines := GetLines("inputs/day21.txt")
	digitRe := regexp.MustCompile(`(?P<n>\d+)`)
	result0 := NamedCaptures(digitRe, lines[0])
	enemyHp, err := strconv.Atoi(result0["n"])
	Check(err)
	result1 := NamedCaptures(digitRe, lines[1])
	enemyDamage, err := strconv.Atoi(result1["n"])
	Check(err)
	result2 := NamedCaptures(digitRe, lines[2])
	enemyArmor, err := strconv.Atoi(result2["n"])
	Check(err)

	type stats struct {
		name   string
		cost   int
		damage int
		armor  int
	}

	// 5 choose 1
	weapons := []stats{
		stats{"Dagger", 8, 4, 0},
		stats{"Shortsword", 10, 5, 0},
		stats{"Warhammer", 25, 6, 0},
		stats{"Longsword", 40, 7, 0},
		stats{"Greataxe", 74, 8, 0},
	}

	// 5 choose 1
	armor := []stats{
		stats{"Nekkid", 0, 0, 0},
		stats{"Leather", 13, 0, 1},
		stats{"Chainmail", 31, 0, 2},
		stats{"Splintmail", 53, 0, 3},
		stats{"Bandedmail", 75, 0, 4},
		stats{"Platemail", 102, 0, 5},
	}

	// 6 choose 0-2
	rings := []stats{
		stats{"Damage +1", 25, 1, 0},
		stats{"Damage +2", 50, 2, 0},
		stats{"Damage +3", 100, 3, 0},
		stats{"Defense +1", 20, 0, 1},
		stats{"Defense +2", 40, 0, 2},
		stats{"Defense +3", 80, 0, 3},
	}

	outfits := []stats{}
	for _, weapon := range weapons {
		for _, armor := range armor {
			stat := stats{
				name:   weapon.name + " " + armor.name,
				cost:   weapon.cost + armor.cost,
				damage: weapon.damage + armor.damage,
				armor:  weapon.armor + armor.armor,
			}
			origStat := stat

			// no ring
			outfits = append(outfits, origStat)

			// 1 ring
			for _, ring := range rings {
				stat := stats{
					name:   origStat.name + " " + ring.name,
					cost:   origStat.cost + ring.cost,
					damage: origStat.damage + ring.damage,
					armor:  origStat.armor + ring.armor,
				}
				outfits = append(outfits, stat)
			}

			// 2 rings
			for i := 0; i < len(rings); i++ {
				for j := 0; j < len(rings); j++ {
					if i >= j {
						continue
					}
					ri := rings[i]
					rj := rings[j]
					stat := stats{
						name:   origStat.name + " " + ri.name + " " + rj.name,
						cost:   origStat.cost + ri.cost + rj.cost,
						damage: origStat.damage + ri.damage + rj.damage,
						armor:  origStat.armor + ri.armor + rj.armor,
					}
					outfits = append(outfits, stat)
				}
			}
		}
	}
	sort.Slice(outfits, func(i, j int) bool {
		return outfits[i].cost < outfits[j].cost
	})

Outfits:
	for _, outfit := range outfits {

		myHp := 100
		theirHp := enemyHp

		for {
			damageToThem := outfit.damage - enemyArmor
			if damageToThem < 0 {
				damageToThem = 0
			}
			theirHp -= damageToThem
			if theirHp <= 0 {
				fmt.Println(outfit)
				fmt.Println(outfit.cost)
				break Outfits
			}

			damageToMe := enemyDamage - outfit.armor
			if damageToMe < 0 {
				damageToMe = 0
			}
			myHp -= damageToMe
			if myHp <= 0 {
				continue Outfits
			}
		}
	}

	// part 2

ReverseOutfits:
	for i := len(outfits) - 1; i > 0; i-- {
		outfit := outfits[i]

		myHp := 100
		theirHp := enemyHp

		for {
			damageToThem := outfit.damage - enemyArmor
			if damageToThem < 0 {
				damageToThem = 0
			}
			theirHp -= damageToThem
			if theirHp <= 0 {
				continue ReverseOutfits
			}

			damageToMe := enemyDamage - outfit.armor
			if damageToMe < 0 {
				damageToMe = 0
			}
			myHp -= damageToMe
			if myHp <= 0 {
				fmt.Println(outfit)
				fmt.Println(outfit.cost)
				break ReverseOutfits
			}
		}
	}

}