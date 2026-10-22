package leetcode

import "testing"

func TestFindKthLargestExampleOne(t *testing.T) {
	if got := findKthLargest([]int{3, 2, 1, 5, 6, 4}, 2); got != 5 {
		t.Fatalf("findKthLargest() = %d, want 5", got)
	}
}

func TestFindKthLargestExampleTwo(t *testing.T) {
	if got := findKthLargest([]int{3, 2, 3, 1, 2, 4, 5, 5, 6}, 4); got != 4 {
		t.Fatalf("findKthLargest() = %d, want 4", got)
	}
}
