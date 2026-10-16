package leetcode

// LeetCode #95 - Unique Binary Search Trees II
//
// Every value in the range can be the root. Build the left and right ranges, then combine them.
func generateTrees(n int) []*TreeNode {
	if n == 0 {
		return nil
	}

	var build func(lo, hi int) []*TreeNode
	build = func(lo, hi int) []*TreeNode {
		if lo > hi {
			return []*TreeNode{nil}
		}
		var trees []*TreeNode
		for root := lo; root <= hi; root++ {
			lefts := build(lo, root-1)
			rights := build(root+1, hi)
			for _, left := range lefts {
				for _, right := range rights {
					trees = append(trees, &TreeNode{
						Val:   root,
						Left:  cloneTree(left),
						Right: cloneTree(right),
					})
				}
			}
		}
		return trees
	}

	return build(1, n)
}

func cloneTree(node *TreeNode) *TreeNode {
	if node == nil {
		return nil
	}
	return &TreeNode{Val: node.Val, Left: cloneTree(node.Left), Right: cloneTree(node.Right)}
}
