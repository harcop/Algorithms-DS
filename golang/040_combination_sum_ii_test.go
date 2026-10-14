package leetcode

import "testing"

func TestCombinationSum2ExampleOne(t *testing.T) {
	got := combinationSum2([]int{10, 1, 2, 7, 6, 1, 5}, 8)
	want := [][]int{{1, 1, 6}, {1, 2, 5}, {1, 7}, {2, 6}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("combinationSum2() = %v, want %v", got, want)
	}
}

func TestCombinationSum2ExampleTwo(t *testing.T) {
	got := combinationSum2([]int{2, 5, 2, 1, 2}, 5)
	want := [][]int{{1, 2, 2}, {5}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("combinationSum2() = %v, want %v", got, want)
	}
}
