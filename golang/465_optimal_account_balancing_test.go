package leetcode

import "testing"

func TestMinTransfersExamples(t *testing.T) {
	if got := minTransfers([][]int{{0, 1, 10}, {2, 0, 5}}); got != 2 {
		t.Fatalf("minTransfers() = %d, want 2", got)
	}
	if got := minTransfers([][]int{{0, 1, 10}, {1, 0, 1}, {1, 2, 5}, {2, 0, 5}}); got != 1 {
		t.Fatalf("minTransfers() = %d, want 1", got)
	}
}
