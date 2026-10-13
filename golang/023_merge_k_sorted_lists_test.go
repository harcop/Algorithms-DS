package leetcode

import "testing"

func TestMergeKListsExampleOne(t *testing.T) {
	lists := []*ListNode{
		vecToList([]int{1, 4, 5}),
		vecToList([]int{1, 3, 4}),
		vecToList([]int{2, 6}),
	}
	got := listToVec(mergeKLists(lists))
	want := []int{1, 1, 2, 3, 4, 4, 5, 6}
	if !sameInts(got, want) {
		t.Fatalf("mergeKLists() = %v, want %v", got, want)
	}
}

func TestMergeKListsExampleTwo(t *testing.T) {
	got := listToVec(mergeKLists(nil))
	if len(got) != 0 {
		t.Fatalf("mergeKLists() = %v, want empty", got)
	}
}

func TestMergeKListsExampleThree(t *testing.T) {
	got := listToVec(mergeKLists([]*ListNode{nil}))
	if len(got) != 0 {
		t.Fatalf("mergeKLists() = %v, want empty", got)
	}
}
