package leetcode

import "testing"

func TestMaxAExamples(t *testing.T) {
	if got := maxA(3); got != 3 {
		t.Fatalf("maxA(3) = %d, want 3", got)
	}
	if got := maxA(7); got != 9 {
		t.Fatalf("maxA(7) = %d, want 9", got)
	}
}
