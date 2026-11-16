package leetcode

import "testing"

func TestUpdateMatrixExample(t *testing.T) {
	got := updateMatrix([][]int{{0, 0, 0}, {0, 1, 0}, {1, 1, 1}})
	want := [][]int{{0, 0, 0}, {0, 1, 0}, {1, 2, 1}}
	if !sameMatrix(got, want) {
		t.Fatalf("updateMatrix = %v, want %v", got, want)
	}
}
