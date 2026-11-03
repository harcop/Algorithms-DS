package leetcode

import "math/rand"

// LeetCode #380 - Insert Delete GetRandom O(1)
//
// A slice stores the values and a map stores each value's index so removal can swap with the last.
type RandomizedSet struct {
	index map[int]int
	vals  []int
}

func NewRandomizedSet() *RandomizedSet {
	return &RandomizedSet{index: map[int]int{}}
}

func (s *RandomizedSet) Insert(val int) bool {
	if _, ok := s.index[val]; ok {
		return false
	}
	s.index[val] = len(s.vals)
	s.vals = append(s.vals, val)
	return true
}

func (s *RandomizedSet) Remove(val int) bool {
	i, ok := s.index[val]
	if !ok {
		return false
	}
	last := s.vals[len(s.vals)-1]
	s.vals[i] = last
	s.index[last] = i
	s.vals = s.vals[:len(s.vals)-1]
	delete(s.index, val)
	return true
}

func (s *RandomizedSet) GetRandom() int {
	return s.vals[rand.Intn(len(s.vals))]
}
