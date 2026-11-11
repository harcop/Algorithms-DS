package leetcode

import "testing"

func TestFindPermutationExamples(t *testing.T) {
	if got := findPermutation("I"); !sameInts(got, []int{1, 2}) {
		t.Fatalf("findPermutation(I) = %v, want [1 2]", got)
	}
	if got := findPermutation("DI"); !sameInts(got, []int{2, 1, 3}) {
		t.Fatalf("findPermutation(DI) = %v, want [2 1 3]", got)
	}
}
