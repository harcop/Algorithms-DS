package leetcode

import "testing"

func TestFindDuplicatesExample(t *testing.T) {
	got := findDuplicates([]int{4, 3, 2, 7, 8, 2, 3, 1})
	want := []int{2, 3}
	if !sameInts(got, want) {
		t.Fatalf("findDuplicates() = %v, want %v", got, want)
	}
}
