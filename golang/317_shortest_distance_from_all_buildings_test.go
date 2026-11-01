package leetcode

import "testing"

func TestShortestDistanceBuildingsExample(t *testing.T) {
	grid := [][]int{{1, 0, 2, 0, 1}, {0, 0, 0, 0, 0}, {0, 0, 1, 0, 0}}
	if got := shortestDistanceBuildings(grid); got != 7 {
		t.Fatalf("shortestDistanceBuildings() = %d, want 7", got)
	}
}
