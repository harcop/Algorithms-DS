package leetcode

// LeetCode #637 - Average of Levels in Binary Tree
//
// Each level's values are averaged before the next level is visited.
func averageOfLevels(root *TreeNode) []float64 {
	if root == nil {
		return nil
	}
	var out []float64
	queue := []*TreeNode{root}
	for len(queue) > 0 {
		size := len(queue)
		sum := 0
		for i := 0; i < size; i++ {
			node := queue[i]
			sum += node.Val
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		queue = queue[size:]
		out = append(out, float64(sum)/float64(size))
	}
	return out
}
