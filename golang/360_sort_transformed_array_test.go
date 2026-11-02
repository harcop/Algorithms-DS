package leetcode

import "testing"

func TestSortTransformedArrayQuadratic(t *testing.T) {
	got := sortTransformedArray([]int{-4, -2, 2, 4}, 1, 3, 5)
	want := []int{3, 9, 15, 33}
	if !sameInts(got, want) {
		t.Fatalf("sortTransformedArray() = %v, want %v", got, want)
	}
}

func TestSortTransformedArrayLinear(t *testing.T) {
	got := sortTransformedArray([]int{0, 1}, 0, 3, 10)
	want := []int{10, 13}
	if !sameInts(got, want) {
		t.Fatalf("sortTransformedArray() = %v, want %v", got, want)
	}
}
