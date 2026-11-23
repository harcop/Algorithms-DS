package leetcode

// LeetCode #654 - Maximum Binary Tree
//
// A decreasing stack keeps the nodes still waiting for a larger parent on their right.
func constructMaximumBinaryTree(nums []int) *TreeNode {
	var stack []*TreeNode
	for _, value := range nums {
		node := &TreeNode{Val: value}
		for len(stack) > 0 && stack[len(stack)-1].Val < value {
			node.Left = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		if len(stack) > 0 {
			stack[len(stack)-1].Right = node
		}
		stack = append(stack, node)
	}
	if len(stack) == 0 {
		return nil
	}
	return stack[0]
}
