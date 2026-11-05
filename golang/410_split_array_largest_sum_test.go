package leetcode

import "testing"

func TestSplitArrayExamples(t *testing.T) {
	if got := splitArray([]int{7, 2, 5, 10, 8}, 2); got != 18 {
		t.Fatalf("splitArray() = %d, want 18", got)
	}
	if got := splitArray([]int{1, 2, 3, 4, 5}, 2); got != 9 {
		t.Fatalf("splitArray() = %d, want 9", got)
	}
}
