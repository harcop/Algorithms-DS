package leetcode

import "testing"

func TestMaxDepthNaryExample(t *testing.T) {
	root := &NaryNode{
		Val: 1,
		Children: []*NaryNode{
			{
				Val: 2,
				Children: []*NaryNode{
					{Val: 5},
					{Val: 6},
				},
			},
			{Val: 3},
			{Val: 4},
		},
	}
	if got := maxDepthNary(root); got != 3 {
		t.Fatalf("maxDepthNary = %d, want 3", got)
	}
}
