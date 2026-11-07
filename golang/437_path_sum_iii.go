package leetcode

// LeetCode #437 - Path Sum III
//
// A prefix-sum count records how often the current sum minus the target has already appeared.
func pathSumIII(root *TreeNode, targetSum int) int {
	count := map[int]int{0: 1}
	var walk func(*TreeNode, int) int
	walk = func(node *TreeNode, sum int) int {
		if node == nil {
			return 0
		}
		sum += node.Val
		paths := count[sum-targetSum]
		count[sum]++
		paths += walk(node.Left, sum) + walk(node.Right, sum)
		count[sum]--
		return paths
	}
	return walk(root, 0)
}
