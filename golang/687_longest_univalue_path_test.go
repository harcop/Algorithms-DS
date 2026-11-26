package leetcode

import "testing"

func TestLongestUnivaluePathExamples(t *testing.T) {
	root := makeTree(5,
		makeTree(4, makeTree(1, nil, nil), makeTree(1, nil, nil)),
		makeTree(5, nil, makeTree(5, nil, nil)),
	)
	if got := longestUnivaluePath(root); got != 2 {
		t.Fatalf("longestUnivaluePath() = %d, want 2", got)
	}
	root = makeTree(1,
		makeTree(4, makeTree(4, nil, nil), makeTree(4, nil, nil)),
		makeTree(5, nil, nil),
	)
	if got := longestUnivaluePath(root); got != 2 {
		t.Fatalf("longestUnivaluePath() = %d, want 2", got)
	}
}
