package leetcode

import "math/rand"

// LeetCode #381 - Insert Delete GetRandom O(1) - Duplicates allowed
//
// Each value keeps every index it occupies so one copy can be swapped out in constant time.
type RandomizedCollection struct {
	pos  map[int][]int
	vals []int
}

func NewRandomizedCollection() *RandomizedCollection {
	return &RandomizedCollection{pos: map[int][]int{}}
}

func (c *RandomizedCollection) Insert(val int) bool {
	indexes := c.pos[val]
	first := len(indexes) == 0
	c.pos[val] = append(indexes, len(c.vals))
	c.vals = append(c.vals, val)
	return first
}

func (c *RandomizedCollection) Remove(val int) bool {
	indexes := c.pos[val]
	if len(indexes) == 0 {
		return false
	}
	i := indexes[len(indexes)-1]
	c.pos[val] = indexes[:len(indexes)-1]
	last := len(c.vals) - 1
	if i != last {
		moved := c.vals[last]
		c.vals[i] = moved
		movedIndexes := c.pos[moved]
		for j := range movedIndexes {
			if movedIndexes[j] == last {
				movedIndexes[j] = i
				break
			}
		}
		c.pos[moved] = movedIndexes
	}
	c.vals = c.vals[:last]
	if len(c.pos[val]) == 0 {
		delete(c.pos, val)
	}
	return true
}

func (c *RandomizedCollection) GetRandom() int {
	return c.vals[rand.Intn(len(c.vals))]
}
