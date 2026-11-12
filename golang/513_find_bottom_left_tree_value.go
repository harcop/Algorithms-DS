package leetcode

// LeetCode #513 - Find Bottom Left Tree Value
//
// Level order keeps the first node of each row, so the last row's first node is the bottom-left value.
func findBottomLeftValue(root *TreeNode) int {
	queue := []*TreeNode{root}
	value := root.Val
	for len(queue) > 0 {
		value = queue[0].Val
		size := len(queue)
		for i := 0; i < size; i++ {
			node := queue[i]
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		queue = queue[size:]
	}
	return value
}
