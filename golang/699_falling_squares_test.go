package leetcode

import "testing"

func TestFallingSquaresExamples(t *testing.T) {
	got := fallingSquares([][]int{{1, 2}, {2, 3}, {6, 1}})
	if !sameInts(got, []int{2, 5, 5}) {
		t.Fatalf("fallingSquares() = %v, want [2 5 5]", got)
	}
	got = fallingSquares([][]int{{100, 100}, {200, 100}})
	if !sameInts(got, []int{100, 100}) {
		t.Fatalf("fallingSquares() = %v, want [100 100]", got)
	}
}
