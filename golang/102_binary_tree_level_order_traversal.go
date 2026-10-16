package leetcode

// LeetCode #102 - Binary Tree Level Order Traversal
//
// Walk the queue one level at a time and enqueue each node's children.
func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	var out [][]int
	queue := []*TreeNode{root}
	for head := 0; head < len(queue); {
		size := len(queue) - head
		level := make([]int, 0, size)
		for i := 0; i < size; i++ {
			node := queue[head]
			head++
			level = append(level, node.Val)
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		out = append(out, level)
	}
	return out
}
