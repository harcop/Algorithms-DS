package leetcode

import "testing"

func TestDeleteDuplicatesExampleOne(t *testing.T) {
	got := listToVec(deleteDuplicates(vecToList([]int{1, 1, 2})))
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("deleteDuplicates() = %v, want %v", got, want)
	}
}

func TestDeleteDuplicatesExampleTwo(t *testing.T) {
	got := listToVec(deleteDuplicates(vecToList([]int{1, 1, 2, 3, 3})))
	want := []int{1, 2, 3}
	if !sameInts(got, want) {
		t.Fatalf("deleteDuplicates() = %v, want %v", got, want)
	}
}
