package leetcode

import "testing"

func TestUniquePathsWithObstaclesExampleOne(t *testing.T) {
	got := uniquePathsWithObstacles([][]int{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}})
	if got != 2 {
		t.Fatalf("uniquePathsWithObstacles() = %d, want 2", got)
	}
}

func TestUniquePathsWithObstaclesExampleTwo(t *testing.T) {
	got := uniquePathsWithObstacles([][]int{{0, 1}, {0, 0}})
	if got != 1 {
		t.Fatalf("uniquePathsWithObstacles() = %d, want 1", got)
	}
}
