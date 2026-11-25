package leetcode

// LeetCode #662 - Maximum Width of Binary Tree
//
// Heap indices mark each level, and subtracting the leftmost index leaves the gap between the ends.
func widthOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}
	type placed struct {
		node *TreeNode
		idx  int
	}
	level := []placed{{root, 0}}
	best := 0
	for len(level) > 0 {
		left := level[0].idx
		best = max(best, level[len(level)-1].idx-left+1)
		next := make([]placed, 0)
		for _, cur := range level {
			id := cur.idx - left
			if cur.node.Left != nil {
				next = append(next, placed{cur.node.Left, id * 2})
			}
			if cur.node.Right != nil {
				next = append(next, placed{cur.node.Right, id*2 + 1})
			}
		}
		level = next
	}
	return best
}
