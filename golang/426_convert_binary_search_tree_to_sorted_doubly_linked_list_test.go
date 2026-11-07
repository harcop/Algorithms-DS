package leetcode

import "testing"

func TestTreeToDoublyListExample(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil)),
		makeTree(5, nil, nil),
	)
	got := circularValues(t, treeToDoublyList(root))
	if !sameInts(got, []int{1, 2, 3, 4, 5}) {
		t.Fatalf("treeToDoublyList() = %v, want [1 2 3 4 5]", got)
	}
	only := treeToDoublyList(makeTree(7, nil, nil))
	if only.Left != only || only.Right != only {
		t.Fatal("single node did not close on itself")
	}
	if treeToDoublyList(nil) != nil {
		t.Fatal("treeToDoublyList(nil) != nil")
	}
}

func circularValues(t *testing.T, head *TreeNode) []int {
	t.Helper()
	var out []int
	cur := head
	for {
		out = append(out, cur.Val)
		if cur.Right == nil || cur.Right.Left != cur {
			t.Fatalf("broken doubly link at %d", cur.Val)
		}
		cur = cur.Right
		if cur == head {
			return out
		}
		if len(out) > 16 {
			t.Fatal("doubly list did not close")
		}
	}
}
