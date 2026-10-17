package leetcode

import "testing"

func TestConnectExampleOne(t *testing.T) {
	n4 := &Node{Val: 4}
	n5 := &Node{Val: 5}
	n6 := &Node{Val: 6}
	n7 := &Node{Val: 7}
	n2 := &Node{Val: 2, Left: n4, Right: n5}
	n3 := &Node{Val: 3, Left: n6, Right: n7}
	root := &Node{Val: 1, Left: n2, Right: n3}

	connect(root)
	if n2.Next != n3 {
		t.Fatalf("connect() node 2 next = %v, want 3", nodeVal(n2.Next))
	}
	if n4.Next != n5 || n5.Next != n6 || n6.Next != n7 {
		t.Fatal("connect() did not link the bottom level left to right")
	}
}

func nodeVal(node *Node) any {
	if node == nil {
		return nil
	}
	return node.Val
}
