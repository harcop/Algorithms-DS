package leetcode

import "testing"

func TestPermuteUniqueExampleOne(t *testing.T) {
	got := permuteUnique([]int{1, 1, 2})
	want := [][]int{{1, 1, 2}, {1, 2, 1}, {2, 1, 1}}
	if !sameIntGroups(got, want) {
		t.Fatalf("permuteUnique() = %v, want %v", got, want)
	}
}

func TestPermuteUniqueExampleTwo(t *testing.T) {
	got := permuteUnique([]int{1, 2, 3})
	if len(got) != 6 {
		t.Fatalf("permuteUnique() = %v, want 6 permutations", got)
	}
}
