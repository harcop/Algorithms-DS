package leetcode

import "strconv"

// LeetCode #257 - Binary Tree Paths
//
// Walk to each leaf and record the nodes joined by "->".
func binaryTreePaths(root *TreeNode) []string {
	var out []string
	var path []string
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		path = append(path, strconv.Itoa(node.Val))
		if node.Left == nil && node.Right == nil {
			out = append(out, joinPath(path))
		} else {
			walk(node.Left)
			walk(node.Right)
		}
		path = path[:len(path)-1]
	}
	walk(root)
	return out
}

func joinPath(path []string) string {
	out := path[0]
	for _, part := range path[1:] {
		out += "->" + part
	}
	return out
}
