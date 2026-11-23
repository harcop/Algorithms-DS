package leetcode

import "testing"

func TestFindLongestChainExamples(t *testing.T) {
	if got := findLongestChain([][]int{{1, 2}, {2, 3}, {3, 4}}); got != 2 {
		t.Fatalf("findLongestChain() = %d, want 2", got)
	}
	if got := findLongestChain([][]int{{1, 2}, {7, 8}, {4, 5}}); got != 3 {
		t.Fatalf("findLongestChain() = %d, want 3", got)
	}
}
