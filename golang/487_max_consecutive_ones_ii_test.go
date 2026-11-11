package leetcode

import "testing"

func TestFindMaxConsecutiveOnesIIExamples(t *testing.T) {
	if got := findMaxConsecutiveOnesII([]int{1, 0, 1, 1, 0}); got != 4 {
		t.Fatalf("findMaxConsecutiveOnesII() = %d, want 4", got)
	}
	if got := findMaxConsecutiveOnesII([]int{1, 0, 1, 1, 0, 1}); got != 4 {
		t.Fatalf("findMaxConsecutiveOnesII() = %d, want 4", got)
	}
}
