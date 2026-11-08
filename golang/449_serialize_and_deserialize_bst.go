package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #449 - Serialize and Deserialize BST
//
// LeetCode names these serialize and deserialize; those names already belong to the general binary tree (#297).
// A preorder list with # for a missing child is enough to rebuild the tree.
func serializeBST(root *TreeNode) string {
	var parts []string
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			parts = append(parts, "#")
			return
		}
		parts = append(parts, strconv.Itoa(node.Val))
		walk(node.Left)
		walk(node.Right)
	}
	walk(root)
	return strings.Join(parts, ",")
}

func deserializeBST(data string) *TreeNode {
	tokens := strings.Split(data, ",")
	i := 0
	var build func() *TreeNode
	build = func() *TreeNode {
		if i >= len(tokens) || tokens[i] == "#" {
			i++
			return nil
		}
		value, _ := strconv.Atoi(tokens[i])
		i++
		node := &TreeNode{Val: value}
		node.Left = build()
		node.Right = build()
		return node
	}
	return build()
}
