package leetcode

import "testing"

func TestSmallestRangeExamples(t *testing.T) {
	got := smallestRange([][]int{{4, 10, 15, 24, 26}, {0, 9, 12, 20}, {5, 18, 22, 30}})
	if !sameInts(got, []int{20, 24}) {
		t.Fatalf("smallestRange() = %v, want [20 24]", got)
	}
	got = smallestRange([][]int{{1, 2, 3}, {1, 2, 3}, {1, 2, 3}})
	if !sameInts(got, []int{1, 1}) {
		t.Fatalf("smallestRange() = %v, want [1 1]", got)
	}
}
