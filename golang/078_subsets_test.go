package leetcode

import "testing"

func TestSubsetsExampleOne(t *testing.T) {
	got := subsets([]int{1, 2, 3})
	want := [][]int{{}, {1}, {2}, {3}, {1, 2}, {1, 3}, {2, 3}, {1, 2, 3}}
	if !sameIntGroups(got, want) {
		t.Fatalf("subsets() = %v, want %v", got, want)
	}
}

func TestSubsetsExampleTwo(t *testing.T) {
	got := subsets([]int{0})
	want := [][]int{{}, {0}}
	if !sameIntGroups(got, want) {
		t.Fatalf("subsets() = %v, want %v", got, want)
	}
}
