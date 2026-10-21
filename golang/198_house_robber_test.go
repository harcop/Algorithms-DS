package leetcode

import "testing"

func TestRobExampleOne(t *testing.T) {
	if got := rob([]int{1, 2, 3, 1}); got != 4 {
		t.Fatalf("rob() = %d, want 4", got)
	}
}

func TestRobExampleTwo(t *testing.T) {
	if got := rob([]int{2, 7, 9, 3, 1}); got != 12 {
		t.Fatalf("rob() = %d, want 12", got)
	}
}
