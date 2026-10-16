package leetcode

import "testing"

func TestIsSymmetricExampleOne(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(3, nil, nil), makeTree(4, nil, nil)),
		makeTree(2, makeTree(4, nil, nil), makeTree(3, nil, nil)),
	)
	if !isSymmetric(root) {
		t.Fatal("isSymmetric() = false, want true")
	}
}

func TestIsSymmetricExampleTwo(t *testing.T) {
	root := makeTree(1,
		makeTree(2, nil, nil),
		makeTree(2, nil, makeTree(3, nil, nil)),
	)
	if isSymmetric(root) {
		t.Fatal("isSymmetric() = true, want false")
	}
}
