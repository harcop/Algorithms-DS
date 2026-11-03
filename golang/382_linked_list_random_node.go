package leetcode

import "math/rand"

// LeetCode #382 - Linked List Random Node
//
// Each call walks the list and keeps a node with probability 1/i at step i.
type LinkedListRandom struct {
	head *ListNode
}

func NewLinkedListRandom(head *ListNode) *LinkedListRandom {
	return &LinkedListRandom{head: head}
}

func (r *LinkedListRandom) GetRandom() int {
	pick := r.head.Val
	node := r.head.Next
	for i := 2; node != nil; i++ {
		if rand.Intn(i) == 0 {
			pick = node.Val
		}
		node = node.Next
	}
	return pick
}
