package leetcode

import "testing"

func TestSearchInsertExampleOne(t *testing.T) {
	got := searchInsert([]int{1, 3, 5, 6}, 5)
	if got != 2 {
		t.Fatalf("searchInsert() = %d, want 2", got)
	}
}

func TestSearchInsertExampleTwo(t *testing.T) {
	got := searchInsert([]int{1, 3, 5, 6}, 2)
	if got != 1 {
		t.Fatalf("searchInsert() = %d, want 1", got)
	}
}

func TestSearchInsertExampleThree(t *testing.T) {
	got := searchInsert([]int{1, 3, 5, 6}, 7)
	if got != 4 {
		t.Fatalf("searchInsert() = %d, want 4", got)
	}
}
