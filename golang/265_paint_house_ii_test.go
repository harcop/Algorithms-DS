package leetcode

import "testing"

func TestMinCostIIExampleOne(t *testing.T) {
	costs := [][]int{{1, 5, 3}, {2, 9, 4}}
	if got := minCostII(costs); got != 5 {
		t.Fatalf("minCostII() = %d, want 5", got)
	}
}
