package leetcode

import "testing"

func TestCutOffTreeExamples(t *testing.T) {
	forest := [][]int{{1, 2, 3}, {0, 0, 4}, {7, 6, 5}}
	if got := cutOffTree(forest); got != 6 {
		t.Fatalf("cutOffTree() = %d, want 6", got)
	}
	blocked := [][]int{{1, 2, 3}, {0, 0, 0}, {7, 6, 5}}
	if got := cutOffTree(blocked); got != -1 {
		t.Fatalf("cutOffTree() = %d, want -1", got)
	}
}
