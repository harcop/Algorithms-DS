package leetcode

// LeetCode #129 - Sum Root to Leaf Numbers
//
// Each step appends a digit. A leaf contributes the number built along its path.
func sumNumbers(root *TreeNode) int {
	var dfs func(node *TreeNode, current int) int
	dfs = func(node *TreeNode, current int) int {
		if node == nil {
			return 0
		}
		current = current*10 + node.Val
		if node.Left == nil && node.Right == nil {
			return current
		}
		return dfs(node.Left, current) + dfs(node.Right, current)
	}
	return dfs(root, 0)
}
