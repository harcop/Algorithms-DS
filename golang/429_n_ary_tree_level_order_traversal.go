package leetcode

// LeetCode #429 - N-ary Tree Level Order Traversal
//
// A queue walks one level of children at a time.
func levelOrderNary(root *NaryNode) [][]int {
	if root == nil {
		return nil
	}
	var levels [][]int
	queue := []*NaryNode{root}
	for len(queue) > 0 {
		size := len(queue)
		level := make([]int, 0, size)
		for i := 0; i < size; i++ {
			node := queue[i]
			level = append(level, node.Val)
			queue = append(queue, node.Children...)
		}
		queue = queue[size:]
		levels = append(levels, level)
	}
	return levels
}
