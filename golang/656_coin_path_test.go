package leetcode

import "testing"

func TestCheapestJumpExamples(t *testing.T) {
	got := cheapestJump([]int{1, 2, 4, -1, 2}, 2)
	if !sameInts(got, []int{1, 3, 5}) {
		t.Fatalf("cheapestJump() = %v, want [1 3 5]", got)
	}
	got = cheapestJump([]int{1, 1, 1, 1}, 2)
	if !sameInts(got, []int{1, 2, 4}) {
		t.Fatalf("cheapestJump() = %v, want [1 2 4]", got)
	}
	if got := cheapestJump([]int{1, 2, 4, -1, 2}, 1); got != nil {
		t.Fatalf("cheapestJump() = %v, want none", got)
	}
}
