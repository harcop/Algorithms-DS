package leetcode

import "testing"

func TestCombinationSum4Example(t *testing.T) {
	if got := combinationSum4([]int{1, 2, 3}, 4); got != 7 {
		t.Fatalf("combinationSum4() = %d, want 7", got)
	}
}
