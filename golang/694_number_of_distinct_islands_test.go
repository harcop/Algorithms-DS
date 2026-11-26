package leetcode

import "testing"

func TestNumDistinctIslandsExamples(t *testing.T) {
	same := [][]int{
		{1, 1, 0, 0, 0},
		{1, 1, 0, 0, 0},
		{0, 0, 0, 1, 1},
		{0, 0, 0, 1, 1},
	}
	if got := numDistinctIslands(same); got != 1 {
		t.Fatalf("numDistinctIslands() = %d, want 1", got)
	}
	distinct := [][]int{
		{1, 1, 0, 1, 1},
		{1, 0, 0, 0, 0},
		{0, 0, 0, 0, 1},
		{1, 1, 0, 1, 1},
	}
	if got := numDistinctIslands(distinct); got != 3 {
		t.Fatalf("numDistinctIslands() = %d, want 3", got)
	}
}
