package leetcode

import "testing"

func TestFindClosestElementsExamples(t *testing.T) {
	got := findClosestElements([]int{1, 2, 3, 4, 5}, 4, 3)
	if !sameInts(got, []int{1, 2, 3, 4}) {
		t.Fatalf("findClosestElements() = %v, want [1 2 3 4]", got)
	}
	got = findClosestElements([]int{1, 2, 3, 4, 5}, 4, -1)
	if !sameInts(got, []int{1, 2, 3, 4}) {
		t.Fatalf("findClosestElements() = %v, want [1 2 3 4]", got)
	}
}
