package leetcode

import "testing"

func TestIntegerBreakExampleOne(t *testing.T) {
	if got := integerBreak(10); got != 36 {
		t.Fatalf("integerBreak() = %d, want 36", got)
	}
}
