package leetcode

import (
	"strconv"
	"strings"
)

// LeetCode #297 - Serialize and Deserialize Binary Tree
//
// Write a preorder list, using # for a missing child, and rebuild from that list.
func serialize(root *TreeNode) string {
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

func deserialize(data string) *TreeNode {
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
