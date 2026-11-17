package leetcode

import "testing"

func TestLongestLineExamples(t *testing.T) {
	mat := [][]int{{0, 1, 1, 0}, {0, 1, 1, 0}, {0, 0, 0, 1}}
	if got := longestLine(mat); got != 3 {
		t.Fatalf("longestLine = %d, want 3", got)
	}
	if got := longestLine([][]int{{1, 1, 1, 1}}); got != 4 {
		t.Fatalf("longestLine = %d, want 4", got)
	}
}
