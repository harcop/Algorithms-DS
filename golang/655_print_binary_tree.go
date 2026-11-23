package leetcode

import "strconv"

// LeetCode #655 - Print Binary Tree
//
// The height fixes the width, and each node sits in the middle of the span left for its subtree.
func printTree(root *TreeNode) [][]string {
	var height func(*TreeNode) int
	height = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		return 1 + max(height(node.Left), height(node.Right))
	}
	levels := height(root)
	width := (1 << levels) - 1
	grid := make([][]string, levels)
	for row := range grid {
		grid[row] = make([]string, width)
	}
	var place func(*TreeNode, int, int)
	place = func(node *TreeNode, row, col int) {
		if node == nil {
			return
		}
		grid[row][col] = strconv.Itoa(node.Val)
		if row == levels-1 {
			return
		}
		offset := 1 << (levels - row - 2)
		place(node.Left, row+1, col-offset)
		place(node.Right, row+1, col+offset)
	}
	place(root, 0, (width-1)/2)
	return grid
}
