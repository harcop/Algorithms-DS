package leetcode

// LeetCode #559 - Maximum Depth of N-ary Tree
//
// LeetCode names this maxDepth; that name already belongs to the binary tree (#104).
// Depth is one plus the deepest child.
func maxDepthNary(root *NaryNode) int {
	if root == nil {
		return 0
	}
	deepest := 0
	for _, child := range root.Children {
		deepest = max(deepest, maxDepthNary(child))
	}
	return deepest + 1
}
