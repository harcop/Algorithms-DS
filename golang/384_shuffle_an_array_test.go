package leetcode

import (
	"sort"
	"testing"
)

func TestShuffleArrayExample(t *testing.T) {
	shuffler := NewShuffleArray([]int{1, 2, 3})
	shuffled := shuffler.Shuffle()
	sorted := append([]int(nil), shuffled...)
	sort.Ints(sorted)
	if !sameInts(sorted, []int{1, 2, 3}) {
		t.Fatalf("Shuffle() = %v, want a permutation of [1 2 3]", shuffled)
	}
	if got := shuffler.Reset(); !sameInts(got, []int{1, 2, 3}) {
		t.Fatalf("Reset() = %v, want [1 2 3]", got)
	}
}
