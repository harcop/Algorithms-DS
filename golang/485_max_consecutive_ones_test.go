package leetcode

import "testing"

func TestFindMaxConsecutiveOnesExample(t *testing.T) {
	if got := findMaxConsecutiveOnes([]int{1, 1, 0, 1, 1, 1}); got != 3 {
		t.Fatalf("findMaxConsecutiveOnes() = %d, want 3", got)
	}
}
