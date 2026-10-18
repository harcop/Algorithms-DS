package leetcode

import "testing"

func TestDetectCycleNoCycle(t *testing.T) {
	if detectCycle(vecToList([]int{1, 2, 3, 4})) != nil {
		t.Fatal("detectCycle() found a cycle, want none")
	}
}

func TestDetectCycleReturnsStart(t *testing.T) {
	start := &ListNode{Val: 2}
	tail := &ListNode{Val: -4, Next: start}
	start.Next = &ListNode{Val: 0, Next: tail}
	head := &ListNode{Val: 3, Next: start}
	if got := detectCycle(head); got != start {
		t.Fatalf("detectCycle() = %v, want the node with value 2", got)
	}
}
