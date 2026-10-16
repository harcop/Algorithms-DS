package leetcode

// LeetCode #98 - Validate Binary Search Tree
//
// Every node must sit strictly between the low and high bounds inherited from its ancestors.
func isValidBST(root *TreeNode) bool {
	var validate func(node *TreeNode, lo, hi *int64) bool
	validate = func(node *TreeNode, lo, hi *int64) bool {
		if node == nil {
			return true
		}
		value := int64(node.Val)
		if (lo != nil && value <= *lo) || (hi != nil && value >= *hi) {
			return false
		}
		return validate(node.Left, lo, &value) && validate(node.Right, &value, hi)
	}
	return validate(root, nil, nil)
}
