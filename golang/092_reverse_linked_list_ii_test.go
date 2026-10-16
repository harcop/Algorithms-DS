package leetcode

import "testing"

func TestReverseBetweenExampleOne(t *testing.T) {
	got := listToVec(reverseBetween(vecToList([]int{1, 2, 3, 4, 5}), 2, 4))
	want := []int{1, 4, 3, 2, 5}
	if !sameInts(got, want) {
		t.Fatalf("reverseBetween() = %v, want %v", got, want)
	}
}

func TestReverseBetweenExampleTwo(t *testing.T) {
	got := listToVec(reverseBetween(vecToList([]int{5}), 1, 1))
	want := []int{5}
	if !sameInts(got, want) {
		t.Fatalf("reverseBetween() = %v, want %v", got, want)
	}
}
