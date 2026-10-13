package leetcode

import "testing"

func TestFourSumExampleOne(t *testing.T) {
	got := fourSum([]int{1, 0, -1, 0, -2, 2}, 0)
	want := [][]int{{-2, -1, 1, 2}, {-2, 0, 0, 2}, {-1, 0, 0, 1}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("fourSum() = %v, want %v", got, want)
	}
}

func TestFourSumExampleTwo(t *testing.T) {
	got := fourSum([]int{2, 2, 2, 2, 2}, 8)
	want := [][]int{{2, 2, 2, 2}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("fourSum() = %v, want %v", got, want)
	}
}
