package leetcode

import "testing"

func TestMaxRotateFunctionExample(t *testing.T) {
	if got := maxRotateFunction([]int{4, 3, 2, 6}); got != 26 {
		t.Fatalf("maxRotateFunction() = %d, want 26", got)
	}
}
