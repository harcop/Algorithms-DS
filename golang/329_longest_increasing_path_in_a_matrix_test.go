package leetcode

import "testing"

func TestLongestIncreasingPathExamples(t *testing.T) {
	if got := longestIncreasingPath([][]int{{9, 9, 4}, {6, 6, 8}, {2, 1, 1}}); got != 4 {
		t.Fatalf("longestIncreasingPath() = %d, want 4", got)
	}
	if got := longestIncreasingPath([][]int{{3, 4, 5}, {3, 2, 6}}); got != 4 {
		t.Fatalf("longestIncreasingPath() = %d, want 4", got)
	}
}
