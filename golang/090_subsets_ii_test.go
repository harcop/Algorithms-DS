package leetcode

import "testing"

func TestSubsetsWithDupExampleOne(t *testing.T) {
	got := subsetsWithDup([]int{1, 2, 2})
	want := [][]int{{}, {1}, {1, 2}, {1, 2, 2}, {2}, {2, 2}}
	if !sameIntGroups(got, want) {
		t.Fatalf("subsetsWithDup() = %v, want %v", got, want)
	}
}

func TestSubsetsWithDupExampleTwo(t *testing.T) {
	got := subsetsWithDup([]int{0})
	want := [][]int{{}, {0}}
	if !sameIntGroups(got, want) {
		t.Fatalf("subsetsWithDup() = %v, want %v", got, want)
	}
}
