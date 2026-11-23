package leetcode

import "testing"

func TestIsPossibleExamples(t *testing.T) {
	if !isPossible([]int{1, 2, 3, 3, 4, 5}) {
		t.Fatal("isPossible([1 2 3 3 4 5]) = false, want true")
	}
	if !isPossible([]int{1, 2, 3, 3, 4, 4, 5, 5}) {
		t.Fatal("isPossible([1 2 3 3 4 4 5 5]) = false, want true")
	}
	if isPossible([]int{1, 2, 3, 4, 4, 5}) {
		t.Fatal("isPossible([1 2 3 4 4 5]) = true, want false")
	}
}
