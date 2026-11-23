package leetcode

import "testing"

func TestConstructMaximumBinaryTreeExample(t *testing.T) {
	got := constructMaximumBinaryTree([]int{3, 2, 1, 6, 0, 5})
	want := makeTree(6,
		makeTree(3, nil, makeTree(2, nil, makeTree(1, nil, nil))),
		makeTree(5, makeTree(0, nil, nil), nil),
	)
	if !isSameTree(got, want) {
		t.Fatal("constructMaximumBinaryTree() does not match the maximum tree")
	}
}
