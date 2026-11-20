package leetcode

import "testing"

func TestTree2StrExamples(t *testing.T) {
	root := makeTree(1, makeTree(2, makeTree(4, nil, nil), nil), makeTree(3, nil, nil))
	if got := tree2str(root); got != "1(2(4))(3)" {
		t.Fatalf("tree2str() = %q, want 1(2(4))(3)", got)
	}

	root = makeTree(1, makeTree(2, nil, makeTree(4, nil, nil)), makeTree(3, nil, nil))
	if got := tree2str(root); got != "1(2()(4))(3)" {
		t.Fatalf("tree2str() = %q, want 1(2()(4))(3)", got)
	}
}
