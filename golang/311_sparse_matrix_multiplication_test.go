package leetcode

import "testing"

func TestMultiplySparseExampleOne(t *testing.T) {
	got := multiplySparse([][]int{{1, 0, 0}, {-1, 0, 3}}, [][]int{{7, 0, 0}, {0, 0, 0}, {0, 0, 1}})
	want := [][]int{{7, 0, 0}, {-7, 0, 3}}
	if !sameMatrix(got, want) {
		t.Fatalf("multiplySparse() = %v, want %v", got, want)
	}
}
