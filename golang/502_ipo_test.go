package leetcode

import "testing"

func TestFindMaximizedCapitalExamples(t *testing.T) {
	if got := findMaximizedCapital(2, 0, []int{1, 2, 3}, []int{0, 1, 1}); got != 4 {
		t.Fatalf("findMaximizedCapital(2, 0) = %d, want 4", got)
	}
	if got := findMaximizedCapital(3, 0, []int{1, 2, 3}, []int{0, 1, 2}); got != 6 {
		t.Fatalf("findMaximizedCapital(3, 0) = %d, want 6", got)
	}
}
