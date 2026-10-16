package leetcode

// LeetCode #106 - Construct Binary Tree from Inorder and Postorder Traversal
//
// LeetCode names this buildTree; that name already belongs to preorder construction (#105).
// The last postorder value is the root. Its inorder index splits the left and right ranges.
func buildTreeFromPostorder(inorder []int, postorder []int) *TreeNode {
	if len(inorder) == 0 {
		return nil
	}
	rootVal := postorder[len(postorder)-1]
	idx := 0
	for i, value := range inorder {
		if value == rootVal {
			idx = i
			break
		}
	}
	return &TreeNode{
		Val:   rootVal,
		Left:  buildTreeFromPostorder(inorder[:idx], postorder[:idx]),
		Right: buildTreeFromPostorder(inorder[idx+1:], postorder[idx:len(postorder)-1]),
	}
}
