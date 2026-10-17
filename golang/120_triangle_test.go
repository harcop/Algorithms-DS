package leetcode

import "testing"

func TestMinimumTotalExampleOne(t *testing.T) {
	got := minimumTotal([][]int{{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}})
	if got != 11 {
		t.Fatalf("minimumTotal() = %d, want 11", got)
	}
}

func TestMinimumTotalExampleTwo(t *testing.T) {
	got := minimumTotal([][]int{{-10}})
	if got != -10 {
		t.Fatalf("minimumTotal() = %d, want -10", got)
	}
}
