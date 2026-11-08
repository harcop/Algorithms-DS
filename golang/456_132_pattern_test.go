package leetcode

import "testing"

func TestFind132patternExamples(t *testing.T) {
	if !find132pattern([]int{3, 1, 4, 2}) {
		t.Fatal("find132pattern([3,1,4,2]) = false, want true")
	}
	if find132pattern([]int{1, 2, 3, 4}) {
		t.Fatal("find132pattern([1,2,3,4]) = true, want false")
	}
}
