package leetcode

import "testing"

func TestFindLengthOfLCISExamples(t *testing.T) {
	if got := findLengthOfLCIS([]int{1, 3, 5, 4, 7}); got != 3 {
		t.Fatalf("findLengthOfLCIS() = %d, want 3", got)
	}
	if got := findLengthOfLCIS([]int{2, 2, 2, 2, 2}); got != 1 {
		t.Fatalf("findLengthOfLCIS() = %d, want 1", got)
	}
}
