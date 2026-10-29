package leetcode

import "testing"

func TestMinTotalDistanceExampleOne(t *testing.T) {
	grid := [][]int{{1, 0, 0, 0, 1}, {0, 0, 0, 0, 0}, {0, 0, 1, 0, 0}}
	if got := minTotalDistance(grid); got != 6 {
		t.Fatalf("minTotalDistance() = %d, want 6", got)
	}
}
