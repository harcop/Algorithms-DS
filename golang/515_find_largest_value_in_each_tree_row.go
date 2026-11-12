package leetcode

// LeetCode #515 - Find Largest Value in Each Tree Row
//
// One pass over each level records that row's maximum.
func largestValues(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var out []int
	queue := []*TreeNode{root}
	for len(queue) > 0 {
		size := len(queue)
		best := queue[0].Val
		for i := 0; i < size; i++ {
			node := queue[i]
			if node.Val > best {
				best = node.Val
			}
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		queue = queue[size:]
		out = append(out, best)
	}
	return out
}
