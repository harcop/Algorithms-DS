package leetcode

import "testing"

func TestNumWaysExampleOne(t *testing.T) {
	if got := numWays(3, 2); got != 6 {
		t.Fatalf("numWays() = %d, want 6", got)
	}
}

func TestNumWaysExampleTwo(t *testing.T) {
	if got := numWays(1, 1); got != 1 {
		t.Fatalf("numWays() = %d, want 1", got)
	}
}
