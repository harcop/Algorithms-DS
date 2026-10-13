package leetcode

import "testing"

func TestMergeTwoListsExampleOne(t *testing.T) {
	l1 := vecToList([]int{1, 2, 4})
	l2 := vecToList([]int{1, 3, 4})
	got := listToVec(mergeTwoLists(l1, l2))
	want := []int{1, 1, 2, 3, 4, 4}
	if !sameInts(got, want) {
		t.Fatalf("mergeTwoLists() = %v, want %v", got, want)
	}
}

func TestMergeTwoListsExampleTwo(t *testing.T) {
	got := listToVec(mergeTwoLists(nil, nil))
	if len(got) != 0 {
		t.Fatalf("mergeTwoLists() = %v, want empty", got)
	}
}

func TestMergeTwoListsExampleThree(t *testing.T) {
	l2 := vecToList([]int{0})
	got := listToVec(mergeTwoLists(nil, l2))
	want := []int{0}
	if !sameInts(got, want) {
		t.Fatalf("mergeTwoLists() = %v, want %v", got, want)
	}
}
