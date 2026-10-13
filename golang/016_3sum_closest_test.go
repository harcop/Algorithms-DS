package leetcode

import "testing"

func TestThreeSumClosestExampleOne(t *testing.T) {
	got := threeSumClosest([]int{-1, 2, 1, -4}, 1)
	if got != 2 {
		t.Fatalf("threeSumClosest() = %d, want 2", got)
	}
}

func TestThreeSumClosestExampleTwo(t *testing.T) {
	got := threeSumClosest([]int{0, 0, 0}, 1)
	if got != 0 {
		t.Fatalf("threeSumClosest() = %d, want 0", got)
	}
}
