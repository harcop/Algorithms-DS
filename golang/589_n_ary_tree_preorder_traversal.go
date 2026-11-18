package leetcode

// LeetCode #589 - N-ary Tree Preorder Traversal
//
// Visit the node, then each child from left to right.
func preorder(root *NaryNode) []int {
	if root == nil {
		return nil
	}
	out := []int{root.Val}
	for _, child := range root.Children {
		out = append(out, preorder(child)...)
	}
	return out
}
