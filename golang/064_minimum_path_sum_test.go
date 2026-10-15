package leetcode

import "testing"

func TestMinPathSumExampleOne(t *testing.T) {
	got := minPathSum([][]int{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}})
	if got != 7 {
		t.Fatalf("minPathSum() = %d, want 7", got)
	}
}

func TestMinPathSumExampleTwo(t *testing.T) {
	got := minPathSum([][]int{{1, 2, 3}, {4, 5, 6}})
	if got != 12 {
		t.Fatalf("minPathSum() = %d, want 12", got)
	}
}
