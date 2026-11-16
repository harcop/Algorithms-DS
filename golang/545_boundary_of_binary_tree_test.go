package leetcode

import "testing"

func TestBoundaryOfBinaryTreeExamples(t *testing.T) {
	small := makeTree(1, nil, makeTree(2, makeTree(3, nil, nil), makeTree(4, nil, nil)))
	if got, want := boundaryOfBinaryTree(small), []int{1, 3, 4, 2}; !sameInts(got, want) {
		t.Fatalf("boundaryOfBinaryTree = %v, want %v", got, want)
	}
	root := makeTree(1,
		makeTree(2,
			makeTree(4, nil, nil),
			makeTree(5, makeTree(7, nil, nil), makeTree(8, nil, nil)),
		),
		makeTree(3,
			makeTree(6, makeTree(9, nil, nil), makeTree(10, nil, nil)),
			nil,
		),
	)
	want := []int{1, 2, 4, 7, 8, 9, 10, 6, 3}
	if got := boundaryOfBinaryTree(root); !sameInts(got, want) {
		t.Fatalf("boundaryOfBinaryTree = %v, want %v", got, want)
	}
}
