package leetcode

import (
	"strconv"
	"strings"
)

// NaryNode is a node in an n-ary tree.
type NaryNode struct {
	Val      int
	Children []*NaryNode
}

// LeetCode #428 - Serialize and Deserialize N-ary Tree
//
// Preorder records each value and how many children follow it.
func serializeNary(root *NaryNode) string {
	if root == nil {
		return ""
	}
	var parts []string
	var walk func(*NaryNode)
	walk = func(node *NaryNode) {
		parts = append(parts, strconv.Itoa(node.Val), strconv.Itoa(len(node.Children)))
		for _, child := range node.Children {
			walk(child)
		}
	}
	walk(root)
	return strings.Join(parts, ",")
}

func deserializeNary(data string) *NaryNode {
	if data == "" {
		return nil
	}
	parts := strings.Split(data, ",")
	i := 0
	var build func() *NaryNode
	build = func() *NaryNode {
		val, _ := strconv.Atoi(parts[i])
		count, _ := strconv.Atoi(parts[i+1])
		i += 2
		node := &NaryNode{Val: val}
		for c := 0; c < count; c++ {
			node.Children = append(node.Children, build())
		}
		return node
	}
	return build()
}
