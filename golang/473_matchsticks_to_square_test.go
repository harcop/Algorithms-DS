package leetcode

import "testing"

func TestMakesquareExamples(t *testing.T) {
	if !makesquare([]int{1, 1, 2, 2, 2}) {
		t.Fatal("makesquare([1,1,2,2,2]) = false, want true")
	}
	if makesquare([]int{3, 3, 3, 3, 4}) {
		t.Fatal("makesquare([3,3,3,3,4]) = true, want false")
	}
}
