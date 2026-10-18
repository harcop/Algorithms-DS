package leetcode

import "testing"

func TestMaxPointsExampleOne(t *testing.T) {
	got := maxPoints([][]int{{1, 1}, {2, 2}, {3, 3}})
	if got != 3 {
		t.Fatalf("maxPoints() = %d, want 3", got)
	}
}

func TestMaxPointsExampleTwo(t *testing.T) {
	got := maxPoints([][]int{{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}})
	if got != 4 {
		t.Fatalf("maxPoints() = %d, want 4", got)
	}
}
