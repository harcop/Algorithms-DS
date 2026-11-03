package leetcode

import "testing"

func TestGetSumExamples(t *testing.T) {
	if got := getSum(1, 2); got != 3 {
		t.Fatalf("getSum(1, 2) = %d, want 3", got)
	}
	if got := getSum(-2, 3); got != 1 {
		t.Fatalf("getSum(-2, 3) = %d, want 1", got)
	}
}
