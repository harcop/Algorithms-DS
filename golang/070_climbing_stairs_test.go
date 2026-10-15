package leetcode

import "testing"

func TestClimbStairsExampleOne(t *testing.T) {
	got := climbStairs(2)
	if got != 2 {
		t.Fatalf("climbStairs() = %d, want 2", got)
	}
}

func TestClimbStairsExampleTwo(t *testing.T) {
	got := climbStairs(3)
	if got != 3 {
		t.Fatalf("climbStairs() = %d, want 3", got)
	}
}
