package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #606 - Construct String from Binary Tree
//
// Preorder writes the value, and an empty left child stays as () only when a right child exists.
func tree2str(root *TreeNode) string {
	if root == nil {
		return ""
	}
	var b strings.Builder
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		b.WriteString(strconv.Itoa(node.Val))
		if node.Left == nil && node.Right == nil {
			return
		}
		b.WriteByte('(')
		if node.Left != nil {
			walk(node.Left)
		}
		b.WriteByte(')')
		if node.Right != nil {
			b.WriteByte('(')
			walk(node.Right)
			b.WriteByte(')')
		}
	}
	walk(root)
	return b.String()
}
