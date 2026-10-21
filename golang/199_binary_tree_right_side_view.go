package leetcode

// LeetCode #199 - Binary Tree Right Side View
//
// The last node of each level is the one visible from the right.
func rightSideView(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var out []int
	queue := []*TreeNode{root}
	for len(queue) > 0 {
		level := len(queue)
		out = append(out, queue[level-1].Val)
		for i := 0; i < level; i++ {
			node := queue[i]
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		queue = queue[level:]
	}
	return out
}
