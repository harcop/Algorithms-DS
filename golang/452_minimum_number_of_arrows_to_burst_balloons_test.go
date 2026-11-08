package leetcode

import "testing"

func TestFindMinArrowShotsExamples(t *testing.T) {
	if got := findMinArrowShots([][]int{{10, 16}, {2, 8}, {1, 6}, {7, 12}}); got != 2 {
		t.Fatalf("findMinArrowShots() = %d, want 2", got)
	}
	if got := findMinArrowShots([][]int{{1, 2}, {3, 4}, {5, 6}, {7, 8}}); got != 4 {
		t.Fatalf("findMinArrowShots() = %d, want 4", got)
	}
}
