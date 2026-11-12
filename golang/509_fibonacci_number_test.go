package leetcode

import "testing"

func TestFibExamples(t *testing.T) {
	if got := fib(2); got != 1 {
		t.Fatalf("fib(2) = %d, want 1", got)
	}
	if got := fib(3); got != 2 {
		t.Fatalf("fib(3) = %d, want 2", got)
	}
	if got := fib(4); got != 3 {
		t.Fatalf("fib(4) = %d, want 3", got)
	}
}
