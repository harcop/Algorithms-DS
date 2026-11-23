package leetcode

import (
	"sort"
	"testing"
)

func TestFindDuplicateSubtreesExample(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, nil, nil), nil),
		makeTree(3, makeTree(2, makeTree(4, nil, nil), nil), makeTree(4, nil, nil)),
	)
	got := subtreeLabels(findDuplicateSubtrees(root))
	want := []string{"2(4)", "4"}
	if !sameStrings(got, want) {
		t.Fatalf("findDuplicateSubtrees() = %v, want %v", got, want)
	}
}

func subtreeLabels(nodes []*TreeNode) []string {
	labels := make([]string, len(nodes))
	for i, node := range nodes {
		labels[i] = tree2str(node)
	}
	sort.Strings(labels)
	return labels
}
