package leetcode

import "testing"

func TestCombinationSumExampleOne(t *testing.T) {
	got := combinationSum([]int{2, 3, 6, 7}, 7)
	want := [][]int{{2, 2, 3}, {7}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("combinationSum() = %v, want %v", got, want)
	}
}

func TestCombinationSumExampleTwo(t *testing.T) {
	got := combinationSum([]int{2, 3, 5}, 8)
	want := [][]int{{2, 2, 2, 2}, {2, 3, 3}, {3, 5}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("combinationSum() = %v, want %v", got, want)
	}
}

func TestCombinationSumExampleThree(t *testing.T) {
	got := combinationSum([]int{2}, 1)
	if len(got) != 0 {
		t.Fatalf("combinationSum() = %v, want none", got)
	}
}
