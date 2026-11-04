package leetcode

import "math/rand"

// LeetCode #398 - Random Pick Index
//
// Each value remembers the indexes where it appears, and a pick chooses one of them.
type RandomPick struct {
	indexes map[int][]int
}

func NewRandomPick(nums []int) *RandomPick {
	indexes := map[int][]int{}
	for i, value := range nums {
		indexes[value] = append(indexes[value], i)
	}
	return &RandomPick{indexes: indexes}
}

func (p *RandomPick) Pick(target int) int {
	choices := p.indexes[target]
	return choices[rand.Intn(len(choices))]
}
