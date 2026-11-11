package leetcode

import "testing"

func TestFindSubsequencesExamples(t *testing.T) {
	got := findSubsequences([]int{4, 6, 7, 7})
	want := [][]int{{4, 6}, {4, 6, 7}, {4, 6, 7, 7}, {4, 7}, {4, 7, 7}, {6, 7}, {6, 7, 7}, {7, 7}}
	if !sameIntGroups(got, want) {
		t.Fatalf("findSubsequences() = %v, want %v", got, want)
	}
	got = findSubsequences([]int{4, 4, 3, 2, 1})
	want = [][]int{{4, 4}}
	if !sameIntGroups(got, want) {
		t.Fatalf("findSubsequences() = %v, want %v", got, want)
	}
}
