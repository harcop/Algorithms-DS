package leetcode

// LeetCode #536 - Construct Binary Tree from String
//
// Read a number, then a parenthesized left child and an optional parenthesized right child.
func str2tree(s string) *TreeNode {
	if s == "" {
		return nil
	}
	i := 0
	var parse func() *TreeNode
	parse = func() *TreeNode {
		if i >= len(s) {
			return nil
		}
		sign := 1
		if s[i] == '-' {
			sign = -1
			i++
		}
		value := 0
		for i < len(s) && s[i] >= '0' && s[i] <= '9' {
			value = value*10 + int(s[i]-'0')
			i++
		}
		node := &TreeNode{Val: value * sign}
		if i < len(s) && s[i] == '(' {
			i++
			node.Left = parse()
			i++
		}
		if i < len(s) && s[i] == '(' {
			i++
			node.Right = parse()
			i++
		}
		return node
	}
	return parse()
}
