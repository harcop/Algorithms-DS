package leetcode

import "strconv"

// LeetCode #652 - Find Duplicate Subtrees
//
// Each subtree gets a serialization, and a shape is reported the second time it appears.
func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	count := map[string]int{}
	var out []*TreeNode
	var walk func(*TreeNode) string
	walk = func(node *TreeNode) string {
		if node == nil {
			return "#"
		}
		key := strconv.Itoa(node.Val) + "," + walk(node.Left) + "," + walk(node.Right)
		count[key]++
		if count[key] == 2 {
			out = append(out, node)
		}
		return key
	}
	walk(root)
	return out
}
