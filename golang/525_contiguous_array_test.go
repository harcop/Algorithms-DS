package leetcode

import "testing"

func TestFindMaxLengthExamples(t *testing.T) {
	if got := findMaxLength([]int{0, 1}); got != 2 {
		t.Fatalf("findMaxLength() = %d, want 2", got)
	}
	if got := findMaxLength([]int{0, 1, 0}); got != 2 {
		t.Fatalf("findMaxLength() = %d, want 2", got)
	}
}
