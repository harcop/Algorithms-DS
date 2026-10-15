package leetcode

import "testing"

func TestMergeExampleOne(t *testing.T) {
	got := merge([][]int{{1, 3}, {2, 6}, {8, 10}, {15, 18}})
	want := [][]int{{1, 6}, {8, 10}, {15, 18}}
	if !sameMatrix(got, want) {
		t.Fatalf("merge() = %v, want %v", got, want)
	}
}

func TestMergeExampleTwo(t *testing.T) {
	got := merge([][]int{{1, 4}, {4, 5}})
	want := [][]int{{1, 5}}
	if !sameMatrix(got, want) {
		t.Fatalf("merge() = %v, want %v", got, want)
	}
}
