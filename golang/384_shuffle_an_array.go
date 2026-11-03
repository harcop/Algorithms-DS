package leetcode

import "math/rand"

// LeetCode #384 - Shuffle an Array
//
// Fisher-Yates swaps each position with an earlier one, and reset restores the original order.
type ShuffleArray struct {
	original []int
	current  []int
}

func NewShuffleArray(nums []int) *ShuffleArray {
	original := append([]int(nil), nums...)
	current := append([]int(nil), nums...)
	return &ShuffleArray{original: original, current: current}
}

func (s *ShuffleArray) Reset() []int {
	s.current = append([]int(nil), s.original...)
	return append([]int(nil), s.current...)
}

func (s *ShuffleArray) Shuffle() []int {
	for i := len(s.current) - 1; i > 0; i-- {
		j := rand.Intn(i + 1)
		s.current[i], s.current[j] = s.current[j], s.current[i]
	}
	return append([]int(nil), s.current...)
}
