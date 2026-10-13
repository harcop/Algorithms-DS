package leetcode

import "container/heap"

// LeetCode #23 - Merge k Sorted Lists
//
// A min-heap of the current heads yields the next node in sorted order.

type listHeap []*ListNode

func (h listHeap) Len() int           { return len(h) }
func (h listHeap) Less(i, j int) bool { return h[i].Val < h[j].Val }
func (h listHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *listHeap) Push(x any) {
	*h = append(*h, x.(*ListNode))
}

func (h *listHeap) Pop() any {
	old := *h
	n := len(old)
	node := old[n-1]
	old[n-1] = nil
	*h = old[:n-1]
	return node
}

func mergeKLists(lists []*ListNode) *ListNode {
	h := &listHeap{}
	for _, node := range lists {
		if node != nil {
			heap.Push(h, node)
		}
	}

	dummy := &ListNode{}
	tail := dummy
	for h.Len() > 0 {
		node := heap.Pop(h).(*ListNode)
		tail.Next = node
		tail = node
		if node.Next != nil {
			heap.Push(h, node.Next)
		}
	}

	return dummy.Next
}
