package leetcode

import "testing"

func TestSingleNonDuplicateExample(t *testing.T) {
	if got := singleNonDuplicate([]int{1, 1, 2, 3, 3, 4, 4, 8, 8}); got != 2 {
		t.Fatalf("singleNonDuplicate() = %d, want 2", got)
	}
	if got := singleNonDuplicate([]int{3, 3, 7, 7, 10, 11, 11}); got != 10 {
		t.Fatalf("singleNonDuplicate() = %d, want 10", got)
	}
}
