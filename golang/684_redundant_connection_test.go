package leetcode

import "testing"

func TestFindRedundantConnectionExamples(t *testing.T) {
	got := findRedundantConnection([][]int{{1, 2}, {1, 3}, {2, 3}})
	if !sameInts(got, []int{2, 3}) {
		t.Fatalf("findRedundantConnection() = %v, want [2 3]", got)
	}
	got = findRedundantConnection([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 4}, {1, 5}})
	if !sameInts(got, []int{1, 4}) {
		t.Fatalf("findRedundantConnection() = %v, want [1 4]", got)
	}
}
