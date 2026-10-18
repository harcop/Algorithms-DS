package leetcode

// RandomNode is a list node that may also point at an arbitrary node.
type RandomNode struct {
	Val    int
	Next   *RandomNode
	Random *RandomNode
}

// LeetCode #138 - Copy List with Random Pointer
//
// Create a copy of every node, then wire each copy's next and random links to the copies.
func copyRandomList(head *RandomNode) *RandomNode {
	if head == nil {
		return nil
	}
	copies := map[*RandomNode]*RandomNode{}
	for cur := head; cur != nil; cur = cur.Next {
		copies[cur] = &RandomNode{Val: cur.Val}
	}
	for cur := head; cur != nil; cur = cur.Next {
		copies[cur].Next = copies[cur.Next]
		copies[cur].Random = copies[cur.Random]
	}
	return copies[head]
}
