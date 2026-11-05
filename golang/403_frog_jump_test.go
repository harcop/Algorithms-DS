package leetcode

import "testing"

func TestCanCrossExamples(t *testing.T) {
	if !canCross([]int{0, 1, 3, 5, 6, 8, 12, 17}) {
		t.Fatal("canCross(true example) = false, want true")
	}
	if canCross([]int{0, 1, 2, 3, 4, 8, 9, 11}) {
		t.Fatal("canCross(false example) = true, want false")
	}
}
