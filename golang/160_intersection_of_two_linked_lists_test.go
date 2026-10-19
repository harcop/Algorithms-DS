package leetcode

import "testing"

func TestGetIntersectionNodeExampleOne(t *testing.T) {
	common := &ListNode{Val: 8, Next: &ListNode{Val: 4, Next: &ListNode{Val: 5}}}
	a := &ListNode{Val: 4, Next: &ListNode{Val: 1, Next: common}}
	b := &ListNode{Val: 5, Next: &ListNode{Val: 6, Next: &ListNode{Val: 1, Next: common}}}
	got := getIntersectionNode(a, b)
	if got != common {
		t.Fatalf("getIntersectionNode() = %v, want the node with value 8", got)
	}
}

func TestGetIntersectionNodeExampleTwo(t *testing.T) {
	a := vecToList([]int{1, 2, 3})
	b := vecToList([]int{4, 5})
	if getIntersectionNode(a, b) != nil {
		t.Fatal("getIntersectionNode() found a node, want none")
	}
}
