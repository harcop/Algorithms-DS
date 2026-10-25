package leetcode

import "testing"

func TestDeleteNodeExample(t *testing.T) {
	head := vecToList([]int{4, 5, 1, 9})
	deleteNode(head.Next)
	got := listToVec(head)
	want := []int{4, 1, 9}
	if !sameInts(got, want) {
		t.Fatalf("deleteNode() left %v, want %v", got, want)
	}
}
