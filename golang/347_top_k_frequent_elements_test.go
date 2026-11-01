package leetcode

import (
	"sort"
	"testing"
)

func TestTopKFrequentExampleOne(t *testing.T) {
	got := topKFrequent([]int{1, 1, 1, 2, 2, 3}, 2)
	sort.Ints(got)
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("topKFrequent() = %v, want %v", got, want)
	}
}
