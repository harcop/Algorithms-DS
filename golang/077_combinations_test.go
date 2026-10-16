package leetcode

import "testing"

func TestCombineExampleOne(t *testing.T) {
	got := combine(4, 2)
	want := [][]int{{1, 2}, {1, 3}, {1, 4}, {2, 3}, {2, 4}, {3, 4}}
	if !sameIntGroups(got, want) {
		t.Fatalf("combine() = %v, want %v", got, want)
	}
}

func TestCombineExampleTwo(t *testing.T) {
	got := combine(1, 1)
	want := [][]int{{1}}
	if !sameIntGroups(got, want) {
		t.Fatalf("combine() = %v, want %v", got, want)
	}
}
