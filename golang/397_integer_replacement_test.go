package leetcode

import "testing"

func TestIntegerReplacementExamples(t *testing.T) {
	if got := integerReplacement(8); got != 3 {
		t.Fatalf("integerReplacement(8) = %d, want 3", got)
	}
	if got := integerReplacement(7); got != 4 {
		t.Fatalf("integerReplacement(7) = %d, want 4", got)
	}
}
