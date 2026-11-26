package leetcode

import "testing"

func TestFindShortestSubArrayExamples(t *testing.T) {
	if got := findShortestSubArray([]int{1, 2, 2, 3, 1}); got != 2 {
		t.Fatalf("findShortestSubArray() = %d, want 2", got)
	}
	if got := findShortestSubArray([]int{1, 2, 2, 3, 1, 4, 2}); got != 6 {
		t.Fatalf("findShortestSubArray() = %d, want 6", got)
	}
}
