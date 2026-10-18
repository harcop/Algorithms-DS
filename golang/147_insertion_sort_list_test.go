package leetcode

import "testing"

func TestInsertionSortListExampleOne(t *testing.T) {
	got := listToVec(insertionSortList(vecToList([]int{4, 2, 1, 3})))
	want := []int{1, 2, 3, 4}
	if !sameInts(got, want) {
		t.Fatalf("insertionSortList() = %v, want %v", got, want)
	}
}

func TestInsertionSortListExampleTwo(t *testing.T) {
	got := listToVec(insertionSortList(vecToList([]int{-1, 5, 3, 4, 0})))
	want := []int{-1, 0, 3, 4, 5}
	if !sameInts(got, want) {
		t.Fatalf("insertionSortList() = %v, want %v", got, want)
	}
}
