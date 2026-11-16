package leetcode

import "testing"

func TestNextGreaterElementIIIExample(t *testing.T) {
	if got := nextGreaterElementIII(12); got != 21 {
		t.Fatalf("nextGreaterElementIII(12) = %d, want 21", got)
	}
}
