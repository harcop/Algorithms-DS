package leetcode

import "testing"

func TestFindUnsortedSubarrayExamples(t *testing.T) {
	if got := findUnsortedSubarray([]int{2, 6, 4, 8, 10, 9, 15}); got != 5 {
		t.Fatalf("findUnsortedSubarray = %d, want 5", got)
	}
	if got := findUnsortedSubarray([]int{1, 2, 3, 4}); got != 0 {
		t.Fatalf("findUnsortedSubarray = %d, want 0", got)
	}
	if got := findUnsortedSubarray([]int{1}); got != 0 {
		t.Fatalf("findUnsortedSubarray = %d, want 0", got)
	}
}
