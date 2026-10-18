package leetcode

import "testing"

func TestReorderListExampleOne(t *testing.T) {
	head := vecToList([]int{1, 2, 3, 4})
	reorderList(head)
	got := listToVec(head)
	want := []int{1, 4, 2, 3}
	if !sameInts(got, want) {
		t.Fatalf("reorderList() = %v, want %v", got, want)
	}
}

func TestReorderListExampleTwo(t *testing.T) {
	head := vecToList([]int{1, 2, 3, 4, 5})
	reorderList(head)
	got := listToVec(head)
	want := []int{1, 5, 2, 4, 3}
	if !sameInts(got, want) {
		t.Fatalf("reorderList() = %v, want %v", got, want)
	}
}
