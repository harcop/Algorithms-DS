package leetcode

// LeetCode #663 - Equal Tree Partition
//
// One edge splits the tree in half when some subtree, other than the whole tree, holds half the total.
func checkEqualTree(root *TreeNode) bool {
	total := equalTreeSum(root)
	if total%2 != 0 {
		return false
	}
	found := false
	var walk func(*TreeNode) int
	walk = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		sum := node.Val + walk(node.Left) + walk(node.Right)
		if node != root && sum == total/2 {
			found = true
		}
		return sum
	}
	walk(root)
	return found
}

func equalTreeSum(node *TreeNode) int {
	if node == nil {
		return 0
	}
	return node.Val + equalTreeSum(node.Left) + equalTreeSum(node.Right)
}
