package leetcode

import "testing"

func TestFlattenExampleOne(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(3, nil, nil), makeTree(4, nil, nil)),
		makeTree(5, nil, makeTree(6, nil, nil)),
	)
	flatten(root)
	got := rightSpine(root)
	want := []int{1, 2, 3, 4, 5, 6}
	if !sameInts(got, want) {
		t.Fatalf("flatten() = %v, want %v", got, want)
	}
}

func TestFlattenExampleTwo(t *testing.T) {
	flatten(nil)
}

func rightSpine(root *TreeNode) []int {
	var out []int
	for root != nil {
		if root.Left != nil {
			return nil
		}
		out = append(out, root.Val)
		root = root.Right
	}
	return out
}
