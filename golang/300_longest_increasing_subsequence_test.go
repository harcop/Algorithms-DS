package leetcode

import "testing"

func TestLengthOfLISExampleOne(t *testing.T) {
	if got := lengthOfLIS([]int{10, 9, 2, 5, 3, 7, 101, 18}); got != 4 {
		t.Fatalf("lengthOfLIS() = %d, want 4", got)
	}
}

func TestLengthOfLISExampleTwo(t *testing.T) {
	if got := lengthOfLIS([]int{0, 1, 0, 3, 2, 3}); got != 4 {
		t.Fatalf("lengthOfLIS() = %d, want 4", got)
	}
}
