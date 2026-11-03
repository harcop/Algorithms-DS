package leetcode

import "testing"

func TestKSmallestPairsExample(t *testing.T) {
	got := kSmallestPairs([]int{1, 7, 11}, []int{2, 4, 6}, 3)
	want := [][]int{{1, 2}, {1, 4}, {1, 6}}
	if !sameMatrix(got, want) {
		t.Fatalf("kSmallestPairs() = %v, want %v", got, want)
	}
}
