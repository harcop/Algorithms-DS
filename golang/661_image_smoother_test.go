package leetcode

import "testing"

func TestImageSmootherExamples(t *testing.T) {
	got := imageSmoother([][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}})
	want := [][]int{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}
	if !sameMatrix(got, want) {
		t.Fatalf("imageSmoother() = %v, want %v", got, want)
	}
	got = imageSmoother([][]int{{100, 200, 100}, {200, 50, 200}, {100, 200, 100}})
	want = [][]int{{137, 141, 137}, {141, 138, 141}, {137, 141, 137}}
	if !sameMatrix(got, want) {
		t.Fatalf("imageSmoother() = %v, want %v", got, want)
	}
}
