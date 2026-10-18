package leetcode

import "testing"

func TestCloneGraphExampleOne(t *testing.T) {
	a := &GraphNode{Val: 1}
	b := &GraphNode{Val: 2}
	c := &GraphNode{Val: 3}
	d := &GraphNode{Val: 4}
	a.Neighbors = []*GraphNode{b, d}
	b.Neighbors = []*GraphNode{a, c}
	c.Neighbors = []*GraphNode{b, d}
	d.Neighbors = []*GraphNode{a, c}

	cloned := cloneGraph(a)
	if cloned == nil || cloned == a || cloned.Val != 1 || len(cloned.Neighbors) != 2 {
		t.Fatalf("cloneGraph() = %+v, want a new node 1 with 2 neighbors", cloned)
	}
}
