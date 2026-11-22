package leetcode

// LeetCode #623 - Add One Row to Tree
//
// Nodes at the chosen depth hang under new parents, and depth 1 replaces the root.
func addOneRow(root *TreeNode, val int, depth int) *TreeNode {
	if depth == 1 {
		return &TreeNode{Val: val, Left: root}
	}
	var walk func(*TreeNode, int)
	walk = func(node *TreeNode, level int) {
		if node == nil {
			return
		}
		if level == depth-1 {
			node.Left = &TreeNode{Val: val, Left: node.Left}
			node.Right = &TreeNode{Val: val, Right: node.Right}
			return
		}
		walk(node.Left, level+1)
		walk(node.Right, level+1)
	}
	walk(root, 1)
	return root
}
