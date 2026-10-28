package leetcode

import "testing"

func TestFindDuplicateExampleOne(t *testing.T) {
	if got := findDuplicate([]int{1, 3, 4, 2, 2}); got != 2 {
		t.Fatalf("findDuplicate() = %d, want 2", got)
	}
}

func TestFindDuplicateExampleTwo(t *testing.T) {
	if got := findDuplicate([]int{3, 1, 3, 4, 2}); got != 3 {
		t.Fatalf("findDuplicate() = %d, want 3", got)
	}
}
