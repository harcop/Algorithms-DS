package leetcode

import "testing"

func TestDeleteDuplicatesIIExampleOne(t *testing.T) {
	got := listToVec(deleteDuplicatesII(vecToList([]int{1, 2, 3, 3, 4, 4, 5})))
	want := []int{1, 2, 5}
	if !sameInts(got, want) {
		t.Fatalf("deleteDuplicatesII() = %v, want %v", got, want)
	}
}

func TestDeleteDuplicatesIIExampleTwo(t *testing.T) {
	got := listToVec(deleteDuplicatesII(vecToList([]int{1, 1, 1, 2, 3})))
	want := []int{2, 3}
	if !sameInts(got, want) {
		t.Fatalf("deleteDuplicatesII() = %v, want %v", got, want)
	}
}
