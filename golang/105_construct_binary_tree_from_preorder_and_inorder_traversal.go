package leetcode

// LeetCode #105 - Construct Binary Tree from Preorder and Inorder Traversal
//
// The first preorder value is the root. Its inorder index splits the left and right ranges.
func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}
	index := make(map[int]int, len(inorder))
	for i, value := range inorder {
		index[value] = i
	}

	var build func(preLo, preHi, inLo, inHi int) *TreeNode
	build = func(preLo, preHi, inLo, inHi int) *TreeNode {
		if preLo > preHi || inLo > inHi {
			return nil
		}
		rootVal := preorder[preLo]
		idx := index[rootVal]
		leftSize := idx - inLo
		return &TreeNode{
			Val:   rootVal,
			Left:  build(preLo+1, preLo+leftSize, inLo, idx-1),
			Right: build(preLo+leftSize+1, preHi, idx+1, inHi),
		}
	}
	n := len(preorder)
	return build(0, n-1, 0, n-1)
}
