package leetcode

import "testing"

func TestIsSameTreeExampleOne(t *testing.T) {
	p := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	q := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	if !isSameTree(p, q) {
		t.Fatal("isSameTree() = false, want true")
	}
}

func TestIsSameTreeExampleTwo(t *testing.T) {
	p := makeTree(1, makeTree(2, nil, nil), nil)
	q := makeTree(1, nil, makeTree(2, nil, nil))
	if isSameTree(p, q) {
		t.Fatal("isSameTree() = true, want false")
	}
}
