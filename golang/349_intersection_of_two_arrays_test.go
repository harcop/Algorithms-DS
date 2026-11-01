package leetcode

import (
	"sort"
	"testing"
)

func TestIntersectionExampleOne(t *testing.T) {
	got := intersection([]int{1, 2, 2, 1}, []int{2, 2})
	sort.Ints(got)
	want := []int{2}
	if !sameInts(got, want) {
		t.Fatalf("intersection() = %v, want %v", got, want)
	}
}
