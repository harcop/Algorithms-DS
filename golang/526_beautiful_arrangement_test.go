package leetcode

import "testing"

func TestCountArrangementExamples(t *testing.T) {
	if got := countArrangement(2); got != 2 {
		t.Fatalf("countArrangement(2) = %d, want 2", got)
	}
	if got := countArrangement(1); got != 1 {
		t.Fatalf("countArrangement(1) = %d, want 1", got)
	}
}
