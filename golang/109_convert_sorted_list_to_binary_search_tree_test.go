package leetcode

import "testing"

func TestSortedListToBSTExampleOne(t *testing.T) {
	root := sortedListToBST(vecToList([]int{-10, -3, 0, 5, 9}))
	if root == nil || root.Val != 0 {
		t.Fatalf("sortedListToBST() = %+v, want root 0", root)
	}
}
