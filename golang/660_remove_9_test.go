package leetcode

import "testing"

func TestNewIntegerExamples(t *testing.T) {
	if got := newInteger(9); got != 10 {
		t.Fatalf("newInteger(9) = %d, want 10", got)
	}
	if got := newInteger(1); got != 1 {
		t.Fatalf("newInteger(1) = %d, want 1", got)
	}
	if got := newInteger(20); got != 22 {
		t.Fatalf("newInteger(20) = %d, want 22", got)
	}
}
