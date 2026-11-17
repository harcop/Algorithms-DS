package leetcode

import "testing"

func TestMatrixReshapeExamples(t *testing.T) {
	got := matrixReshape([][]int{{1, 2}, {3, 4}}, 1, 4)
	want := [][]int{{1, 2, 3, 4}}
	if !sameMatrix(got, want) {
		t.Fatalf("matrixReshape = %v, want %v", got, want)
	}
	original := [][]int{{1, 2}, {3, 4}}
	got = matrixReshape(original, 2, 4)
	if !sameMatrix(got, original) {
		t.Fatalf("matrixReshape = %v, want the original matrix", got)
	}
}
