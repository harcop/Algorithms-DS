package leetcode

import "testing"

func TestMinCostExampleOne(t *testing.T) {
	costs := [][]int{{17, 2, 17}, {16, 16, 5}, {14, 3, 19}}
	if got := minCost(costs); got != 10 {
		t.Fatalf("minCost() = %d, want 10", got)
	}
}

func TestMinCostExampleTwo(t *testing.T) {
	if got := minCost([][]int{{7, 6, 2}}); got != 2 {
		t.Fatalf("minCost() = %d, want 2", got)
	}
}
