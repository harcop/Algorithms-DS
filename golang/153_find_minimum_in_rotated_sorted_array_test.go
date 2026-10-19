package leetcode

import "testing"

func TestFindMinExampleOne(t *testing.T) {
	got := findMin([]int{3, 4, 5, 1, 2})
	if got != 1 {
		t.Fatalf("findMin() = %d, want 1", got)
	}
}

func TestFindMinExampleTwo(t *testing.T) {
	got := findMin([]int{4, 5, 6, 7, 0, 1, 2})
	if got != 0 {
		t.Fatalf("findMin() = %d, want 0", got)
	}
}

func TestFindMinExampleThree(t *testing.T) {
	got := findMin([]int{11, 13, 15, 17})
	if got != 11 {
		t.Fatalf("findMin() = %d, want 11", got)
	}
}
