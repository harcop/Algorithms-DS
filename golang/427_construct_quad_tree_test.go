package leetcode

import "testing"

func TestConstructQuadTreeExamples(t *testing.T) {
	root := constructQuadTree([][]int{{0, 1}, {1, 0}})
	if root.IsLeaf {
		t.Fatal("mixed grid produced a leaf")
	}
	if !root.TopLeft.IsLeaf || root.TopLeft.Val {
		t.Fatalf("top left = leaf %v val %v, want leaf false", root.TopLeft.IsLeaf, root.TopLeft.Val)
	}
	if !root.TopRight.IsLeaf || !root.TopRight.Val {
		t.Fatal("top right should be a true leaf")
	}
	if !root.BottomLeft.IsLeaf || !root.BottomLeft.Val {
		t.Fatal("bottom left should be a true leaf")
	}
	if !root.BottomRight.IsLeaf || root.BottomRight.Val {
		t.Fatal("bottom right should be a false leaf")
	}

	leaf := constructQuadTree([][]int{{1, 1}, {1, 1}})
	if !leaf.IsLeaf || !leaf.Val || leaf.TopLeft != nil {
		t.Fatal("uniform grid should be one true leaf")
	}
}
