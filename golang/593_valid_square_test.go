package leetcode

import "testing"

func TestValidSquareExamples(t *testing.T) {
	if !validSquare([]int{0, 0}, []int{1, 0}, []int{0, 1}, []int{1, 1}) {
		t.Fatal("validSquare(unit) = false, want true")
	}
	if validSquare([]int{0, 0}, []int{1, 1}, []int{1, 0}, []int{0, 2}) {
		t.Fatal("validSquare(not a square) = true, want false")
	}
	if validSquare([]int{0, 0}, []int{0, 0}, []int{0, 0}, []int{0, 0}) {
		t.Fatal("validSquare(duplicate origin) = true, want false")
	}
}
