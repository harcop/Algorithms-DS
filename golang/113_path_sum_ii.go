package leetcode

// LeetCode #113 - Path Sum II
//
// Record the path to each leaf whose values add up to the target.
func pathSum(root *TreeNode, targetSum int) [][]int {
	var result [][]int
	var path []int

	var dfs func(node *TreeNode, remain int)
	dfs = func(node *TreeNode, remain int) {
		if node == nil {
			return
		}
		path = append(path, node.Val)
		remain -= node.Val
		if node.Left == nil && node.Right == nil {
			if remain == 0 {
				result = append(result, append([]int(nil), path...))
			}
		} else {
			dfs(node.Left, remain)
			dfs(node.Right, remain)
		}
		path = path[:len(path)-1]
	}

	dfs(root, targetSum)
	return result
}
