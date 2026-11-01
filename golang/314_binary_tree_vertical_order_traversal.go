package leetcode

// LeetCode #314 - Binary Tree Vertical Order Traversal
//
// Walk level by level and group values by their column, from left to right.
func verticalOrder(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}
	columns := map[int][]int{}
	type placed struct {
		node *TreeNode
		col  int
	}
	queue := []placed{{root, 0}}
	minCol, maxCol := 0, 0
	for head := 0; head < len(queue); head++ {
		current := queue[head]
		columns[current.col] = append(columns[current.col], current.node.Val)
		if current.col < minCol {
			minCol = current.col
		}
		if current.col > maxCol {
			maxCol = current.col
		}
		if current.node.Left != nil {
			queue = append(queue, placed{current.node.Left, current.col - 1})
		}
		if current.node.Right != nil {
			queue = append(queue, placed{current.node.Right, current.col + 1})
		}
	}
	out := make([][]int, 0, maxCol-minCol+1)
	for col := minCol; col <= maxCol; col++ {
		out = append(out, columns[col])
	}
	return out
}
