package leetcode

import "testing"

func TestMaxCountExamples(t *testing.T) {
	if got := maxCount(3, 3, [][]int{{2, 2}, {3, 3}}); got != 4 {
		t.Fatalf("maxCount = %d, want 4", got)
	}
	if got := maxCount(3, 3, nil); got != 9 {
		t.Fatalf("maxCount = %d, want 9", got)
	}
}
