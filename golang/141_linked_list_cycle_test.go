package leetcode

import "testing"

func TestHasCycleExampleOne(t *testing.T) {
	if hasCycle(vecToList([]int{3, 2, 0, -4})) {
		t.Fatal("hasCycle() = true, want false")
	}
}

func TestHasCycleExampleTwo(t *testing.T) {
	if hasCycle(vecToList([]int{1, 2})) {
		t.Fatal("hasCycle() = true, want false")
	}
}

func TestHasCycleExampleThree(t *testing.T) {
	if hasCycle(vecToList([]int{1})) {
		t.Fatal("hasCycle() = true, want false")
	}
}

func TestHasCycleLoop(t *testing.T) {
	tail := &ListNode{Val: -4}
	head := &ListNode{Val: 3, Next: &ListNode{Val: 2, Next: &ListNode{Val: 0, Next: tail}}}
	tail.Next = head.Next
	if !hasCycle(head) {
		t.Fatal("hasCycle() = false, want true")
	}
}
