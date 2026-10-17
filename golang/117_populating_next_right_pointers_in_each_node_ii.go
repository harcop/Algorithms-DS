package leetcode

// LeetCode #117 - Populating Next Right Pointers in Each Node II
//
// LeetCode names this connect; that name already belongs to the perfect-tree version (#116).
// Stitch the next level together while walking the links already set on this level.
func connectII(root *Node) *Node {
	for level := root; level != nil; {
		var nextHead, nextTail *Node
		for cur := level; cur != nil; cur = cur.Next {
			for _, child := range []*Node{cur.Left, cur.Right} {
				if child == nil {
					continue
				}
				if nextHead == nil {
					nextHead = child
					nextTail = child
					continue
				}
				nextTail.Next = child
				nextTail = child
			}
		}
		level = nextHead
	}
	return root
}
