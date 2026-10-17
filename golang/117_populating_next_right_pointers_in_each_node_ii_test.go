package leetcode

import "testing"

func TestConnectIIExampleOne(t *testing.T) {
	n4 := &Node{Val: 4}
	n5 := &Node{Val: 5}
	n7 := &Node{Val: 7}
	n2 := &Node{Val: 2, Left: n4, Right: n5}
	n3 := &Node{Val: 3, Right: n7}
	root := &Node{Val: 1, Left: n2, Right: n3}

	connectII(root)
	if n4.Next != n5 || n5.Next != n7 {
		t.Fatal("connectII() did not link 4 -> 5 -> 7")
	}
}
