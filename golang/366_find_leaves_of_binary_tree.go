package leetcode

// LeetCode #366 - Find Leaves of Binary Tree
//
// A node's removal round is one more than the taller child, so leaves share the first list.
func findLeaves(root *TreeNode) [][]int {
	var out [][]int
	var height func(*TreeNode) int
	height = func(node *TreeNode) int {
		if node == nil {
			return -1
		}
		h := height(node.Left)
		if right := height(node.Right); right > h {
			h = right
		}
		h++
		if h == len(out) {
			out = append(out, nil)
		}
		out[h] = append(out[h], node.Val)
		return h
	}
	height(root)
	return out
}
