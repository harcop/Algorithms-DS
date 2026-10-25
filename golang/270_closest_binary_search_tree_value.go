package leetcode

import "math"

// LeetCode #270 - Closest Binary Search Tree Value
//
// Walk the search path and keep the value whose distance to the target is smaller.
func closestValue(root *TreeNode, target float64) int {
	best := root.Val
	for root != nil {
		if math.Abs(float64(root.Val)-target) < math.Abs(float64(best)-target) {
			best = root.Val
		}
		if target < float64(root.Val) {
			root = root.Left
		} else {
			root = root.Right
		}
	}
	return best
}
