package leetcode

// LeetCode #590 - N-ary Tree Postorder Traversal
//
// Visit each child from left to right, then the node.
func postorder(root *NaryNode) []int {
	if root == nil {
		return nil
	}
	var out []int
	for _, child := range root.Children {
		out = append(out, postorder(child)...)
	}
	out = append(out, root.Val)
	return out
}
