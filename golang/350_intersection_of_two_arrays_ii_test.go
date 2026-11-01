package leetcode

import (
	"sort"
	"testing"
)

func TestIntersectExampleOne(t *testing.T) {
	got := intersect([]int{1, 2, 2, 1}, []int{2, 2})
	sort.Ints(got)
	want := []int{2, 2}
	if !sameInts(got, want) {
		t.Fatalf("intersect() = %v, want %v", got, want)
	}
}
