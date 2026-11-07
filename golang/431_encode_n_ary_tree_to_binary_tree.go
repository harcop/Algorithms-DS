package leetcode

// LeetCode #431 - Encode N-ary Tree to Binary Tree
//
// The first child becomes the left link, and the remaining siblings hang off the right links.
func encodeNary(root *NaryNode) *TreeNode {
	if root == nil {
		return nil
	}
	bin := &TreeNode{Val: root.Val}
	var prev *TreeNode
	for _, child := range root.Children {
		encoded := encodeNary(child)
		if prev == nil {
			bin.Left = encoded
		} else {
			prev.Right = encoded
		}
		prev = encoded
	}
	return bin
}

func decodeNary(root *TreeNode) *NaryNode {
	if root == nil {
		return nil
	}
	node := &NaryNode{Val: root.Val}
	for child := root.Left; child != nil; child = child.Right {
		node.Children = append(node.Children, decodeNary(child))
	}
	return node
}
