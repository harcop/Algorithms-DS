package leetcode

import "testing"

func TestLastRemainingExamples(t *testing.T) {
	if got := lastRemaining(9); got != 6 {
		t.Fatalf("lastRemaining(9) = %d, want 6", got)
	}
	if got := lastRemaining(1); got != 1 {
		t.Fatalf("lastRemaining(1) = %d, want 1", got)
	}
}
