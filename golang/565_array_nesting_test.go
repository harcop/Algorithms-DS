package leetcode

import "testing"

func TestArrayNestingExample(t *testing.T) {
	if got := arrayNesting([]int{5, 4, 0, 3, 1, 6, 2}); got != 4 {
		t.Fatalf("arrayNesting = %d, want 4", got)
	}
	if got := arrayNesting([]int{0, 1, 2}); got != 1 {
		t.Fatalf("arrayNesting = %d, want 1", got)
	}
}
