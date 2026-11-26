package leetcode

import "testing"

func TestFindRedundantDirectedConnectionExamples(t *testing.T) {
	got := findRedundantDirectedConnection([][]int{{1, 2}, {1, 3}, {2, 3}})
	if !sameInts(got, []int{2, 3}) {
		t.Fatalf("findRedundantDirectedConnection() = %v, want [2 3]", got)
	}
	got = findRedundantDirectedConnection([][]int{{1, 2}, {2, 3}, {3, 4}, {4, 1}, {1, 5}})
	if !sameInts(got, []int{4, 1}) {
		t.Fatalf("findRedundantDirectedConnection() = %v, want [4 1]", got)
	}
	got = findRedundantDirectedConnection([][]int{{2, 1}, {3, 1}, {4, 2}, {1, 4}})
	if !sameInts(got, []int{2, 1}) {
		t.Fatalf("findRedundantDirectedConnection() = %v, want [2 1]", got)
	}
}
