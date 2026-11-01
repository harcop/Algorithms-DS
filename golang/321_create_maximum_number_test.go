package leetcode

import "testing"

func TestMaxNumberExampleOne(t *testing.T) {
	got := maxNumber([]int{3, 4, 6, 5}, []int{9, 1, 2, 5, 8, 3}, 5)
	want := []int{9, 8, 6, 5, 3}
	if !sameInts(got, want) {
		t.Fatalf("maxNumber() = %v, want %v", got, want)
	}
}

func TestMaxNumberExampleTwo(t *testing.T) {
	got := maxNumber([]int{6, 7}, []int{6, 0, 4}, 5)
	want := []int{6, 7, 6, 0, 4}
	if !sameInts(got, want) {
		t.Fatalf("maxNumber() = %v, want %v", got, want)
	}
}
