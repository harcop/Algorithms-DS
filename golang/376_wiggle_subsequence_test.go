package leetcode

import "testing"

func TestWiggleMaxLengthExamples(t *testing.T) {
	if got := wiggleMaxLength([]int{1, 7, 4, 9, 2, 5}); got != 6 {
		t.Fatalf("wiggleMaxLength() = %d, want 6", got)
	}
	if got := wiggleMaxLength([]int{1, 17, 5, 10, 13, 15, 10, 5, 16, 8}); got != 7 {
		t.Fatalf("wiggleMaxLength() = %d, want 7", got)
	}
	if got := wiggleMaxLength([]int{1, 2, 3, 4}); got != 2 {
		t.Fatalf("wiggleMaxLength() = %d, want 2", got)
	}
}
