package leetcode

import "testing"

func TestNthSuperUglyNumberExampleOne(t *testing.T) {
	if got := nthSuperUglyNumber(12, []int{2, 7, 13, 19}); got != 32 {
		t.Fatalf("nthSuperUglyNumber() = %d, want 32", got)
	}
}
