package leetcode

import "testing"

func TestMaxDistanceExamples(t *testing.T) {
	if got := maxDistance([][]int{{1, 2, 3}, {4, 5}, {1, 2, 3}}); got != 4 {
		t.Fatalf("maxDistance() = %d, want 4", got)
	}
	if got := maxDistance([][]int{{1}, {1}}); got != 0 {
		t.Fatalf("maxDistance() = %d, want 0", got)
	}
}
