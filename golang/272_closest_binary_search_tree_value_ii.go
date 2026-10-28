package leetcode

import "math"

// LeetCode #272 - Closest Binary Search Tree Value II
//
// Inorder puts the values in order, then two pointers pick the k closest to the target.
func closestKValues(root *TreeNode, target float64, k int) []int {
	if k == 0 {
		return nil
	}
	var values []int
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		values = append(values, node.Val)
		walk(node.Right)
	}
	walk(root)
	right := 0
	for right < len(values) && float64(values[right]) < target {
		right++
	}
	left := right - 1
	out := make([]int, 0, k)
	for len(out) < k {
		pickLeft := false
		switch {
		case right >= len(values):
			pickLeft = true
		case left < 0:
			pickLeft = false
		default:
			pickLeft = math.Abs(target-float64(values[left])) <= math.Abs(float64(values[right])-target)
		}
		if pickLeft {
			out = append(out, values[left])
			left--
		} else {
			out = append(out, values[right])
			right++
		}
	}
	return out
}
